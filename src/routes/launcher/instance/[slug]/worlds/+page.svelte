<script lang="ts">
	import { page } from "$app/state";
	import { commands, type World } from "$lib/bindings";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { formatDistanceToNow } from "date-fns";
	import { onMount } from "svelte";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Dialog from "$lib/components/ui/dialog";
	import { Folder, MoreHorizontalIcon, Trash2 } from "@lucide/svelte";
	import { toast } from "svelte-sonner";

	async function fetchWorlds(slug: string): Promise<World[]> {
		const res = await commands.getWorlds(slug);
		if (res.status === "ok") {
			return res.data;
		}
		throw new Error(res.error);
	}

	let worlds: World[] = $state([]);
	let worldsPromise: Promise<World[]> = $state(Promise.resolve([]));
	const instanceSlug = page.params.slug;
	let deleteDialogOpen = $state(false);

	onMount(() => {
		worldsPromise = fetchWorlds(instanceSlug).then((data) => {
			worlds = data;
			return data;
		});
	});

	async function openWorldsDir() {
		await commands.openWorldsDir(instanceSlug);
	}

	async function openWorldDir(worldName: string) {
		await commands.openWorldDir(instanceSlug, worldName);
	}

	async function deleteWorld(worldName: string) {
		const res = await commands.deleteWorld(instanceSlug, worldName);
		if (res.status === "ok") {
			worlds = worlds.filter((world) => world.levelName !== worldName);
			deleteDialogOpen = false;
		} else {
			toast.error(res.error);
			throw new Error(res.error);
		}
	}

	function formatLastPlayed(lastPlayed: string | null | undefined): string {
		if (!lastPlayed) {
			return "Never played";
		}
		try {
			const date = new Date(lastPlayed);
			return formatDistanceToNow(date, { addSuffix: true });
		} catch (e) {
			console.error("Error formatting date:", lastPlayed, e);
			return "Invalid date";
		}
	}
</script>

{#snippet worldCard(world: World)}
	<div class="relative flex h-24 overflow-hidden rounded-lg border border-zinc-300 bg-white shadow-lg dark:border-zinc-700 dark:bg-zinc-800">
		<div class="aspect-square h-full w-auto flex-shrink-0 overflow-hidden bg-zinc-200 dark:bg-zinc-700">
			{#if world.icon}
				<img src={world.icon} alt="Icon for {world.levelName}" class="block h-full w-full object-cover" loading="lazy" />
			{/if}
		</div>
		<div class="flex flex-grow flex-col justify-center overflow-hidden p-2 pr-10">
			<h3 class="truncate font-semibold text-zinc-800 dark:text-zinc-100" title={world.levelName}>
				{world.levelName}
			</h3>
			<p class="mt-1 truncate text-xs text-zinc-500 dark:text-zinc-400">
				{formatLastPlayed(world.lastPlayed)}
			</p>
		</div>
		<div class="absolute right-1 top-1">
			<DropdownMenu.Trigger class={buttonVariants({ variant: "ghost" })}>
				<MoreHorizontalIcon class="h-4 w-4" />
				<DropdownMenu.Content>
					<DropdownMenu.Item onclick={() => openWorldDir(world.folderName)}>
						<Folder class="size-4" />
						<span class="sr-only ml-2 sm:not-sr-only">Open Folder</span>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />

					<DropdownMenu.Item
						onclick={() => (deleteDialogOpen = true)}
						class="text-red-600 hover:!text-red-600 focus:!text-red-600 dark:text-red-500 dark:hover:!text-red-500 dark:focus:!text-red-500">
						<div class="flex items-center gap-2">
							<Trash2 class="size-4" />
							<span class="sr-only ml-2 sm:not-sr-only">Delete World</span>
						</div>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Trigger>
		</div>

		<Dialog.Root open={deleteDialogOpen}>
			<Dialog.Content>
				<Dialog.Title>Delete World</Dialog.Title>
				<Dialog.Description>
					Are you sure you want delete the world <strong>{world.levelName}</strong>
					? This action cannot be undone and will permanently delete the world files.
				</Dialog.Description>
				<Dialog.Footer>
					<Button variant="destructive" onclick={() => deleteWorld(world.folderName)}>Delete</Button>
				</Dialog.Footer>
			</Dialog.Content>
		</Dialog.Root>
	</div>
{/snippet}

<div class="relative h-full w-full overflow-y-auto bg-background p-4 text-foreground lg:px-6 lg:py-5">
	{#await worldsPromise}
		<div class="flex h-full items-center justify-center">
			<p class="text-lg text-zinc-400">Loading worlds...</p>
		</div>
	{:then}
		{#if worlds.length > 0}
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each worlds as world}
					<DropdownMenu.Root>
						{@render worldCard(world)}
					</DropdownMenu.Root>
				{/each}
			</div>

			<Button class="fixed bottom-6 right-6" onclick={openWorldsDir} title="Open Worlds Folder">
				<Folder />
				<span class="sr-only ml-2 sm:not-sr-only">Open Folder</span>
			</Button>
		{:else}
			<div class="flex h-full flex-col items-center justify-center">
				<p class="text-center text-lg text-zinc-500 dark:text-zinc-400">No worlds found for this instance.</p>
				<p class="text-center text-sm text-zinc-400 dark:text-zinc-500">Create a new world in Minecraft to see it here.</p>
			</div>
		{/if}
	{:catch error}
		<div class="flex h-full items-center justify-center">
			<p class="text-lg text-red-500">Failed to load worlds: {error.message}</p>
		</div>
	{/await}
</div>
