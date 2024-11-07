export type Payload = {
	message: string;
};

export type LoginDetailsEvent = {
	code: string;
	uri: string;
};

export type JavaDownloadPaths = string[];

export type DownloadState = "none" | "downloading" | "done";

export type ExtractState = "none" | "extracting" | "done";

export type JavaProgress = {
	download: { [key: number]: number };
	extract: { [key: number]: number };
};

export type ProgressEvent = {
	percentage: number;
};

export type Instance = {
	game: {
		modloader: {
			loader: string;
			version: string;
		};
		version: string;
	};
	java: {
		jvm_arguments: string[];
		path: string;
	};
	name: string;
	slug: string;
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
