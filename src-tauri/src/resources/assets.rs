use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use super::version::VersionManifest;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetIndex {
    objects: HashMap<String, AssetObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetObject {
    hash: String,
    size: u64,
}

pub struct AssetManager {
    client: Client,
    assets_dir: PathBuf,
    indexes_dir: PathBuf,
    objects_dir: PathBuf,
    libraries_dir: PathBuf,
}

impl AssetManager {
    pub fn new(client: Client, config_dir: &Path) -> Self {
        let assets_dir = config_dir.join("assets");
        let indexes_dir = assets_dir.join("indexes");
        let objects_dir = assets_dir.join("objects");
        let libraries_dir = config_dir.join("libraries");

        Self {
            client,
            assets_dir,
            indexes_dir,
            objects_dir,
            libraries_dir,
        }
    }

    pub async fn download_assets(&self, version_manifest: &VersionManifest) -> Result<(), Error> {
        create_dir_all(&self.assets_dir).await?;
        create_dir_all(&self.indexes_dir).await?;
        create_dir_all(&self.objects_dir).await?;

        let asset_index = self
            .client
            .get(&version_manifest.asset_index.url)
            .send()
            .await?
            .json::<AssetIndex>()
            .await?;

        let index_path = self
            .indexes_dir
            .join(format!("{}.json", version_manifest.asset_index.id));

        if !index_path.exists() {
            let mut file = File::create(&index_path).await?;
            let index_content = serde_json::to_string(&asset_index)?;
            file.write_all(index_content.as_bytes()).await?;
        }

        let total_assets = asset_index.objects.len();
        info!("Downloading {} assets", total_assets);

        for (_asset_name, asset_object) in asset_index.objects {
            let hash = &asset_object.hash;
            let subdir = &hash[..2];
            let asset_path = self.objects_dir.join(subdir).join(hash);

            if !asset_path.exists() {
                create_dir_all(asset_path.parent().unwrap()).await?;

                let url = format!(
                    "https://resources.download.minecraft.net/{}/{}",
                    subdir, hash
                );
                let mut response = self.client.get(&url).send().await?;
                let mut file = File::create(&asset_path).await?;
                while let Some(chunk) = response.chunk().await? {
                    file.write_all(&chunk).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn download_libraries(
        &self,
        version_manifest: &VersionManifest,
    ) -> Result<(), Error> {
        create_dir_all(&self.libraries_dir).await?;

        for library in &version_manifest.libraries {
            // info!("Downloading library: {}", library.name);
            let artifact = &library.downloads.artifact;
            let library_path = self.libraries_dir.join(&artifact.path);

            if !library_path.exists() {
                create_dir_all(library_path.parent().unwrap()).await?;

                let mut response = self.client.get(&artifact.url).send().await?;
                let mut file = File::create(&library_path).await?;
                while let Some(chunk) = response.chunk().await? {
                    file.write_all(&chunk).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn download_version_jar(
        &self,
        version_manifest: &VersionManifest,
    ) -> Result<(), Error> {
        let versions_dir = self
            .assets_dir
            .parent()
            .unwrap()
            .join("versions")
            .join(&version_manifest.id);
        create_dir_all(&versions_dir).await?;

        let jar_path = versions_dir.join(format!("{}.jar", version_manifest.id));

        if !jar_path.exists() {
            info!("Downloading Minecraft version JAR: {}", version_manifest.id);

            let mut response = self
                .client
                .get(&version_manifest.downloads.client.url)
                .send()
                .await?;
            let mut file = File::create(&jar_path).await?;
            while let Some(chunk) = response.chunk().await? {
                file.write_all(&chunk).await?;
            }
        } else {
            info!(
                "Minecraft version JAR already downloaded: {}",
                version_manifest.id
            );
        }

        Ok(())
    }
}
