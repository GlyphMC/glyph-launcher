<script lang="ts">
	import Cog6Tooth from "svelte-heros-v2/Cog6Tooth.svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { writeText } from "@tauri-apps/plugin-clipboard-manager";
	import { open } from "@tauri-apps/plugin-shell";
	import type { Instance, LoginDetailsEvent, MinecraftProfile } from "../types";

	let instances = $state<Instance[]>([]);
	let profiles = $state<MinecraftProfile[]>([]);
	let selectedProfile = $state<MinecraftProfile>();
	let loginCode = $state("");
	let verificationUri = $state("");
	let showPopUp = $state(false);

	async function fetchInstances() {
		await invoke<Instance[]>("get_instances").then((data) => (instances = data));
	}

	async function getMinecraftProfiles() {
		await invoke<MinecraftProfile[]>("get_minecraft_profiles").then((data) => {
			profiles = data;
			selectedProfile = data[0];
		});
	}

	async function login() {
		await invoke<MinecraftProfile>("login")
			.then((data) => {
				profiles.push(data);
				selectedProfile = data;
			})
			.finally(() => (showPopUp = false));
	}

	listen<LoginDetailsEvent>("login-details", (event) => {
		loginCode = event.payload.code;
		verificationUri = event.payload.uri;
		showPopUp = true;
	});

	async function copyAndOpen() {
		await writeText(loginCode);
		await open(verificationUri);
	}

	$effect(() => {
		console.log(loginCode, verificationUri);
	});

	fetchInstances();
	getMinecraftProfiles();

</script>

{#snippet SidebarCard(href: string, imgSrc?: string, header?: string, subtitle?: string)}
	<a {href} class="group flex gap-4 no-underline">
		{#if imgSrc}
			<img class="inline h-8 w-8 flex-none self-center rounded shadow-md group-hover:brightness-125" src={imgSrc} alt="" />
		{:else}
			<div class="inline h-8 w-8 flex-none self-center rounded bg-zinc-200 shadow-md group-hover:brightness-125"></div>
		{/if}

		<div>
			<header class="self-center font-display font-bold text-zinc-100 duration-150 group-hover:text-zinc-50">{header}</header>
			<p class="font-display text-xs font-bold text-zinc-200">{subtitle}</p>
		</div>
	</a>
{/snippet}

{#if showPopUp}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 backdrop-blur-md">
		<div class="relative w-full max-w-sm rounded-lg bg-zinc-700 p-6 text-center shadow-lg">
			<h2 class="text-lg font-bold text-zinc-50">Login Verification</h2>
			<p class="mt-2 font-bold text-zinc-50">Please enter the following code:</p>
			<p class="my-4 font-mono text-xl text-zinc-50">{loginCode}</p>
			<button class="mt-4 w-full rounded-md bg-green-600 px-4 py-2 font-bold text-zinc-50 hover:bg-green-700" onclick={copyAndOpen}>
				Copy and Open
			</button>
		</div>
	</div>
{/if}

<div class="fixed top-0 flex h-screen max-h-full w-80 flex-col gap-y-4 bg-zinc-500 p-6">
	<div class="flex items-center justify-between text-zinc-100">
		<a href="/">
			<p class="font-display font-bold hover:text-zinc-50">Glyph Launcher</p>
		</a>
		<a href="/settings">
			<Cog6Tooth class="h-6 w-6 text-zinc-100 hover:text-zinc-50" />
		</a>
	</div>

	<div class="flex w-full gap-4">
		<input class="font-vold w-full rounded-md bg-zinc-300 px-4 py-1.5 placeholder:text-zinc-200" type="text" placeholder="Search instances..." />
	</div>

	<div class="flex gap-4">
		<button
			class="w-full rounded-md bg-zinc-300 px-10 py-1.5 font-display font-bold text-zinc-100 shadow-lg shadow-zinc-300/30 duration-150 hover:bg-zinc-200 hover:shadow-zinc-200/50"
			onclick={() => console.log("Button clicked")}>
			Add Instance
		</button>
	</div>

	<div class="flex flex-col gap-y-4 overflow-y-auto overscroll-contain">
		{#each instances as instance}
			{@render SidebarCard(
				`/instance/${instance.slug}`,
				undefined,
				instance.name,
				`${instance.game.modloader.loader.charAt(0).toUpperCase()}${instance.game.modloader.loader.slice(1)} ${instance.game.version}`
			)}
		{/each}
	</div>

	<div class="fixed bottom-0 left-0 flex w-80 justify-between bg-zinc-500 px-6 py-4">
		{#if selectedProfile}
			{@render SidebarCard("/", `https://crafatar.com/avatars/${selectedProfile.id}`, selectedProfile.name, selectedProfile.id)}
		{:else}
			<button
				onclick={login}
				class="w-full rounded-md bg-green-600 px-10 py-1.5 font-display font-bold text-zinc-50 shadow-lg shadow-green-700/50 duration-150 hover:bg-green-700 hover:shadow-green-900/100">
				Login
			</button>
		{/if}
	</div>
</div>
