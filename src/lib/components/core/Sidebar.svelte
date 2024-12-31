<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { Separator } from "$lib/components/ui/separator";
	import type { Instance, InstanceConfig, LoginDetailsEvent, MinecraftProfile } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import ChevronUp from "lucide-svelte/icons/chevron-up";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";
	import LoginPopUp from "./LoginPopUp.svelte";

	let instances = $state<Instance[]>([]);
	let searchInput = $state("");
	let filteredInstances = $state<Instance[]>([]);
	let profiles = $state<MinecraftProfile[]>([]);
	let selectedProfile = $state<MinecraftProfile>();
	let loginCode = $state("");
	let verificationUri = $state("");
	let showPopUp = $state(false);

	async function fetchInstances() {
		await invoke<InstanceConfig>("get_instances").then((data) => {
			instances = data.instances;
			filterInstances();
		});
	}

	function filterInstances() {
		if (searchInput.trim() === "") {
			filteredInstances = instances;
		} else {
			filteredInstances = instances.filter((instance) => instance.name.toLowerCase().includes(searchInput.toLowerCase()));
		}
	}

	$effect(() => filterInstances());

	listen("instance-list-updated", () => {
		fetchInstances();
	});

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

	async function logout() {
		console.log("logout");
	}

	onMount(async () => {
		await fetchInstances();
		await getMinecraftProfiles();
	});
</script>

{#if showPopUp}
	<LoginPopUp {loginCode} {verificationUri} onCancel={() => (showPopUp = false)} />
{/if}

<Sidebar.Root collapsible="icon">
	<div class="flex h-screen flex-col gap-y-4 p-4">
		<Sidebar.Header class="text-zinc-100 hover:text-zinc-50">
			<a href="/#/" class="font-bold">Glyph Launcher</a>
			<Input type="text" placeholder="Search instances..." bind:value={searchInput} />
			<Button variant="outline" onclick={() => goto("/#/launcher/instance/new")}>Add instance</Button>
		</Sidebar.Header>

		<Sidebar.Content class="mx-2 mt-0">
			<ScrollArea class="h-96 w-full rounded-md border">
				<div class="p-4">
					<h4 class="mb-4 text-sm font-bold leading-none">Instances</h4>
					{#each filteredInstances as instance}
						<a href="/#/instance/{instance.slug}">
							<div>
								<p class="text-sm">{instance.name}</p>
							</div>
						</a>
						<Separator class="my-2" />
					{/each}
				</div>
			</ScrollArea>
		</Sidebar.Content>

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
								<DropdownMenu.Item onclick={() => goto("/#/launcher/accounts")}>
									<span>Accounts</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => goto("/#/launcher/settings")}>
									<span>Settings</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={logout}>
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
