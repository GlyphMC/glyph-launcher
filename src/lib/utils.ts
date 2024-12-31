import { invoke } from "@tauri-apps/api/core";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export async function saveJavaToConfig(paths: string[]) {
	await invoke("save_java_to_config", { paths }).then(() => {
		console.log("Java saved to config successfully");
	});
}
