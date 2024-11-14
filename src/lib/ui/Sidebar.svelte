<script lang="ts">
	import { Button } from "$lib/components/ui/button/index";
	import { Input } from "$lib/components/ui/input/index";
	import * as Sidebar from "$lib/components/ui/sidebar/index";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index";
	import * as Card from "$lib/components/ui/card/index";
	import type { Instance, LoginDetailsEvent, MinecraftProfile } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { writeText } from "@tauri-apps/plugin-clipboard-manager";
	import ChevronUp from "lucide-svelte/icons/chevron-up";
	import { goto } from "$app/navigation";

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
		await writeText(loginCode).then(() => open(verificationUri));
	}

	$effect(() => {
		console.log(loginCode, verificationUri);
	});

	fetchInstances();
	getMinecraftProfiles();
</script>

{#if showPopUp}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 backdrop-blur-md">
		<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
			<Card.Header>
				<h2 class="text-lg font-bold text-zinc-50">Login Verification</h2>
				<p class="mt-2 font-bold text-zinc-50">Please enter the following code:</p>
			</Card.Header>
			<Card.Content>
				<p class="my-4 font-mono text-xl text-zinc-50">{loginCode}</p>
			</Card.Content>
			<Card.Footer>
				<Button onclick={copyAndOpen} variant="outline">Copy and Open</Button>
			</Card.Footer>
		</Card.Root>
	</div>
{/if}

<Sidebar.Root collapsible="icon" data-tauri-drag-region>
	<div class="flex h-screen flex-col gap-y-4 p-4">
		<Sidebar.Header class="text-zinc-100 hover:text-zinc-50">
			<a href="/" class="font-bold">Glyph Launcher</a>
			<Input type="text" placeholder="Search instances..." />
			<Button variant="outline">Add instance</Button>
		</Sidebar.Header>

		<Sidebar.Content></Sidebar.Content>

		<Sidebar.Footer>
			{#if selectedProfile}
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								{#snippet child({ props })}
									<Sidebar.MenuButton
										{...props}
										class="flex items-center gap-2 font-bold data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground">
										<img
											src={`https://crafatar.com/avatars/${selectedProfile?.id}`}
											alt="Profile picture of {selectedProfile?.name}"
											class="h-6 w-6 rounded border border-black" />
										{selectedProfile?.name}
										<ChevronUp class="ml-auto" />
									</Sidebar.MenuButton>
								{/snippet}
							</DropdownMenu.Trigger>
							<DropdownMenu.Content side="top" class="w-[--bits-dropdown-menu-anchor-width]">
								<DropdownMenu.Item onclick={() => (goto("/accounts"))}>
									<span>Accounts</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => (goto("/settings"))}>
									<span>Settings</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item>
									<span>Logout</span>
								</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					</Sidebar.MenuItem>
				</Sidebar.Menu>
			{:else}
				<Button variant="default" onclick={login}>Login</Button>
			{/if}
		</Sidebar.Footer>
	</div>
</Sidebar.Root>
