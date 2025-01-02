import { invoke } from "@tauri-apps/api/core";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export async function saveJavaToConfig(paths: string[], automatic: boolean) {
	await invoke("save_java_to_config", { paths, automatic }).then(() => {
		console.log("Java saved to config successfully");
	});
}
