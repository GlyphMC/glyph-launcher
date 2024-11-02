export type Payload = {
	message: string;
}

export type LoginDetails = {
	code: string;
	uri: string;
}

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
}

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
}
