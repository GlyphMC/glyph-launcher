#![cfg(target_os = "windows")]

use anyhow::{Error, Result, anyhow};
use winreg::{
    RegKey,
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
};

#[derive(Debug)]
pub enum GpuPreference {
    Integrated,
    Discrete,
}

pub fn set_gpu_preference(java_path: &str, preference: GpuPreference) -> Result<(), Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\DirectX\\UserGpuPreferences")?;

    let absolute_path = dunce::canonicalize(java_path)?
        .to_string_lossy()
        .replace('/', "\\");

    let value = match preference {
        GpuPreference::Integrated => "GpuPreference=1;",
        GpuPreference::Discrete => "GpuPreference=2;",
    };

    key.set_value(&absolute_path, &value)
        .map_err(|e| anyhow!("Failed to write GPU preference to registry: {}", e))?;

    Ok(())
}

pub fn delete_gpu_preference(java_path: &str) -> Result<(), Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\DirectX\\UserGpuPreferences",
        KEY_WRITE,
    )?;

    let absolute_path = dunce::canonicalize(java_path)?
        .to_string_lossy()
        .replace('/', "\\");

    key.delete_value(&absolute_path)
        .map_err(|e| anyhow!("Failed to delete GPU preference from registry: {}", e))?;

    Ok(())
}
