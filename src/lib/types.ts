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
	distribution: string;
	expectedVersion: number;
	versionMismatch: boolean;
};

export type ProgressEvent = {
	percentage: number;
};

export type InstanceConfig = {
	instances: Instance[];
};

export type Instance = {
	name: string;
	slug: string;
	game: {
		version: string;
		modloader: {
			loader: string;
			version: string;
		};
		url: string;
	};
	java: {
		path: string;
		args: string[];
		version: string;
	};
	settings: {
		hasLaunched: boolean;
		richPresence: boolean;
		maximised: boolean;
		memory: number;
		windowWidth: number;
		windowHeight: number;
	};
};

export type JavaConfig = {
	java8Path: string;
	java17Path: string;
	java21Path: string;
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

export type Version = {
	id: string;
	type: string;
	url: string;
	time: string;
	releaseTime: string;
	sha1: string;
	complianceLevel: number;
};

export type Modloader = "Vanilla" | "Forge" | "Neoforge" | "Fabric" | "";
