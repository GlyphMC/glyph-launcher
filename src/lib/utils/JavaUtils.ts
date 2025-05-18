import { commands } from "$lib/bindings";

export async function saveJavaToConfig(paths: string[], automatic: boolean) {
	if (!paths || paths.length !== 3) return;
	const pathsForConfig: [string, string, string] = [paths[0], paths[1], paths[2]];
	await commands.saveJavaToConfig(pathsForConfig, automatic).then((res) => {
		if (res.status === "ok") {
			console.log("Java saved to config successfully");
		} else {
			console.error("Failed to save Java to config:", res.error);
		}
	});
}

export type JavaDownloadState = "none" | "downloading" | "done";

export type JavaExtractState = "none" | "extracting" | "done";

export type JavaProgress = {
	download: { [key: number]: number };
	extract: { [key: number]: number };
};

export type DownloadPaths = {
	paths: string[];
};

export type ManualJava = {
	version: 8 | 17 | 21;
	path: string;
};

export type JavaTestInfo = {
	valid: boolean;
	version: string;
	vendor: string;
	expectedVersion: number;
	versionMismatch: boolean;
};

export type ProgressEvent = {
	percentage: number;
};
