export type Payload = {
	message: string;
};

export type LoginDetailsEvent = {
	code: string;
	uri: string;
};

export type JavaPaths = string[];

export type DownloadState = "none" | "downloading" | "done";

export type ExtractState = "none" | "extracting" | "done";

export type JavaProgress = {
	download: { [key: number]: number };
	extract: { [key: number]: number };
};

export type ProgressEvent = {
	percentage: number;
};

export type InstanceConfig = {
	instances: Instance[];
}

export type Instance = {
	name: string;
	slug: string;
	game: {
		version: string;
		modloader: {
			loader: string;
			version: string;
		};
	};
	java: {
		path: string;
		args: string[];
	};
	settings: {
		hasLaunched : boolean;
		richPresence: boolean;
		minimized: boolean;
		memory: number;
	};
};

export type MinecraftProfile = {
	id: string;
	name: string;
	skins: [
		{
			id: string;
			state: string;
			url: string;
			variant: string;
		}
	];
	capes: [
		{
			id: string;
			state: string;
			url: string;
			alias: string;
		}
	];
};
