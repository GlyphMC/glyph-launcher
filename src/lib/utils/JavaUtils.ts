import { invoke } from "@tauri-apps/api/core";

export async function saveJavaToConfig(paths: string[], automatic: boolean) {
	await invoke("save_java_to_config", { paths, automatic }).then(() => {
		console.log("Java saved to config successfully");
	});
}
