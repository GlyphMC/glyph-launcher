<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { Separator } from "$lib/components/ui/separator";
	import { ChevronUp } from "@lucide/svelte";
	import { onDestroy, onMount } from "svelte";
	import LoginPopUp from "./LoginPopUp.svelte";
	import { SidebarController } from "$lib/controllers/SidebarController.svelte";
	import { authService } from "$lib/services/AuthService.svelte";
	import { useAvatar } from "$lib/utils/AvatarUtils";

	const controller = new SidebarController();

	onMount(async () => await controller.init());
	onDestroy(() => controller.cleanup());
</script>

{#if authService.showLoginPopUp}
	<LoginPopUp
		loginCode={authService.loginCode}
		verificationUri={authService.verificationUri}
		onCancel={async () => await controller.handleCancelLoginPopUp()} />
{/if}

<Sidebar.Root collapsible="icon">
	<div class="flex h-screen flex-col gap-y-4 p-4">
		<Sidebar.Header class="text-zinc-100 hover:text-zinc-50">
			<a href="/#/launcher" class="font-bold">Glyph Launcher</a>
			<Input type="text" placeholder="Search instances..." bind:value={controller.searchInput} />
			<Button variant="outline" onclick={controller.handleAddInstanceClick}>Add instance</Button>
		</Sidebar.Header>

		<Sidebar.Content class="mx-2 mt-0">
			<ScrollArea class="h-96 w-full rounded-md border">
				<div class="p-4">
					<h4 class="mb-4 text-sm font-bold leading-none">Instances</h4>
					{#each controller.filteredInstances() as instance (instance.slug)}
						<a href="/#/launcher/instance/{instance.slug}">
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
			{#if controller.selectedProfile}
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								{#snippet child({ props })}
									<Sidebar.MenuButton
										{...props}
										class="flex items-center gap-2 font-bold data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground">
										{#await useAvatar(controller.selectedProfile?.id)}
											<div class="h-6 w-6 animate-pulse rounded border border-black bg-zinc-700"></div>
										{:then avatarSrc}
											{#if avatarSrc}
												<img
													src={avatarSrc}
													alt="Profile picture of {controller.selectedProfile?.name}"
													class="h-6 w-6 rounded border border-black" />
											{:else}
												<div class="h-6 w-6 rounded border border-black bg-zinc-700"></div>
											{/if}
										{:catch _}
											<div class="h-6 w-6 rounded border border-black bg-zinc-700"></div>
										{/await}

										{controller.selectedProfile?.name}
										<ChevronUp class="ml-auto" />
									</Sidebar.MenuButton>
								{/snippet}
							</DropdownMenu.Trigger>
							<DropdownMenu.Content side="top" class="w-[--bits-dropdown-menu-anchor-width]">
								<DropdownMenu.Item onclick={() => controller.navigateToAccounts()}>
									<span>Accounts</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => controller.navigateToSettings()}>
									<span>Settings</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => controller.logout()}>
									<span>Logout</span>
								</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					</Sidebar.MenuItem>
				</Sidebar.Menu>
			{:else}
				<Button variant="default" onclick={() => controller.login()}>Login</Button>
			{/if}
		</Sidebar.Footer>
	</div>
</Sidebar.Root>
