import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export async function saveJavaToConfig(paths: string[], automatic: boolean) {
	if (automatic) {
		paths = paths.map((path) => (platform() === "windows" ? path + "\\bin\\javaw.exe" : path + "/bin/java"));
	}
	await invoke("save_java_to_config", { paths }).then(() => {
		console.log("Java saved to config successfully");
	});
}
