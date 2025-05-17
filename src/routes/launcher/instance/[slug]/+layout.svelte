<script lang="ts">
	import { onMount, type Snippet } from "svelte";
	import type { LayoutData } from "./$types";
	import type { Instance } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import Clock from "lucide-svelte/icons/clock";
	import CalendarClock from "lucide-svelte/icons/calendar-clock";
	import SquareChevronRight from "lucide-svelte/icons/square-chevron-right";
	import X from "lucide-svelte/icons/x";
	import { Button } from "$lib/components/ui/button";
	import InstanceAssetsDownloadPopUp from "$lib/components/core/InstanceAssetsDownloadPopUp.svelte";
	import { formatDistanceToNow, parseISO } from "date-fns";
	import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import type { Attachment } from "svelte/attachments";
	import { page } from "$app/state";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	let instance = $state<Instance>();
	let isRunning = $state(false);

	let navLinksContainer = $state<HTMLElement | null>(null);
	let underlineLeft = $state(0);
	let underlineWidth = $state(0);
	let initialUpdateDone = $state(false);

	function updateUnderlinePosition(useViewTransition = true) {
		if (!navLinksContainer) return;

		const updateStyles = () => {
			if (!navLinksContainer) return;
			const activeLink = navLinksContainer.querySelector<HTMLAnchorElement>('a[aria-current="page"]');
			if (activeLink) {
				const activeLinkRect = activeLink.getBoundingClientRect();
				const containerRect = navLinksContainer.getBoundingClientRect();

				underlineLeft = activeLinkRect.left - containerRect.left;
				underlineWidth = activeLinkRect.width;
			} else {
				underlineLeft = 0;
				underlineWidth = 0;
			}
		};

		if (useViewTransition && document.startViewTransition) {
			document.startViewTransition(() => {
				updateStyles();
			});
		} else {
			updateStyles();
		}
	}

	onMount(async () => {
		await getInstance();
		requestAnimationFrame(() => {
			updateUnderlinePosition(false);
			initialUpdateDone = true;
		});
	});

	$effect(() => {
		if (navLinksContainer && initialUpdateDone) {
			const _currentHash = page.url.hash;
			updateUnderlinePosition(true);
		}
	});

	function formatTimePlayed(totalSeconds: number): string {
		if (totalSeconds === undefined || totalSeconds === null || totalSeconds === 0) return "Never";
		if (totalSeconds < 60) {
			return `${totalSeconds} sec played`;
		}
		const minutes = Math.floor(totalSeconds / 60);
		const hours = Math.floor(minutes / 60);
		const remainingMinutes = minutes % 60;

		if (hours > 0) {
			return `${hours} hr ${remainingMinutes} min played`;
		}
		return `${minutes} min played`;
	}

	function formatLastPlayed(isoTimestamp?: string | null): string {
		if (!isoTimestamp) {
			return "Never";
		}

		try {
			const date = parseISO(isoTimestamp);
			return formatDistanceToNow(date, { addSuffix: true });
		} catch (e) {
			console.error("Error parsing lastPlayed date:", e);
			return "Invalid date";
		}
	}

	async function launchInstance() {
		try {
			await invoke("launch_instance", { slug: data.slug });
		} catch (error) {
			console.error("Failed to launch instance:", error);
		} finally {
			await getInstance();
		}
	}

	async function killInstance() {
		if (!isRunning) return;
		console.log("Not implemented yet");
		// try {
		// 	await invoke("kill_instance", { slug: data.slug });
		// } catch (error) {
		// 	console.error("Failed to kill instance:", error);
		// }
	}

	async function getInstance() {
		await invoke<Instance>("get_instance", { slug: data.slug }).then((data) => {
			instance = data;
		});
	}

	async function openLogWindow() {
		console.log("Opening log window");
		const label = `${instance?.slug}-log`.replaceAll(".", "_");
		let logWebview = await WebviewWindow.getByLabel(label);

		if (logWebview) {
			try {
				await logWebview.show();
				await logWebview.setFocus();
			} catch (error) {
				console.error("Error showing log window:", error);
			}
		} else {
			logWebview = new WebviewWindow(label, {
				url: `/#/launcher/instance/${data.slug}/log`,
				resizable: true,
				alwaysOnTop: false,
				visible: true,
				title: `${instance?.name} - Logs`,
				width: 750,
				height: 400,
				minWidth: 750,
				decorations: false
			});
		}

		await logWebview.once("tauri://window-created", () => console.log("Log webview created"));
		await logWebview.once("tauri://error", (e) => console.error("Log webview error:", e));
	}

	const instanceStateHandler: Attachment = () => {
		let unlistenFns: UnlistenFn[] = [];

		const setupEventListeners = async () => {
			const label = `${instance?.slug}`.replaceAll(".", "_");

			const instanceStartedEvent = `${label}-instance-started`;
			const instanceStoppedEvent = `${label}-instance-stopped`;

			unlistenFns.push(
				await listen(instanceStartedEvent, () => {
					console.log("Instance started event received");
					isRunning = true;
				})
			);

			unlistenFns.push(
				await listen(instanceStoppedEvent, () => {
					console.log("Instance stopped event received");
					isRunning = false;
				})
			);
		};

		setupEventListeners();

		return () => unlistenFns.forEach((unlisten) => unlisten());
	};
</script>

<InstanceAssetsDownloadPopUp />

<div {@attach instanceStateHandler} class="w-full overflow-hidden font-display">
	<p class="mb-2 px-10 pt-10 text-3xl font-bold text-zinc-50">{instance?.name}</p>

	<div class="px-10 text-sm text-zinc-300">
		<div class="flex items-center">
			<Clock class="mr-2 size-4" />
			<p>Time played: {formatTimePlayed(instance?.settings?.timePlayed ?? 0)}</p>
		</div>

		<div class="mt-1 flex items-center">
			<CalendarClock class="mr-2 size-4" />
			<p>Last played: {formatLastPlayed(instance?.settings?.lastPlayed)}</p>
		</div>
	</div>

	<div class="ml-10 mt-8 flex space-x-2">
		{#if isRunning}
			<Button class="w-30 bg-red-600 px-10 hover:bg-red-700" onclick={killInstance}>
				<X class="mr-2 size-4" /> Stop
			</Button>
		{:else}
			<Button class="w-30 px-10" onclick={launchInstance} disabled={isRunning}>Launch</Button>
		{/if}
		<Button variant="outline" class="w-24 px-10" onclick={openLogWindow}>
			<SquareChevronRight /> Logs
		</Button>
	</div>

	<div class="relative mt-5 px-10 text-xl" bind:this={navLinksContainer}>
		<div class="flex space-x-10">
			{#each data.sections as section}
				{@const sectionPath = `#/launcher/instance/${data.slug}/${section.slug}`}
				{@const isActive = page.url.hash === sectionPath}
				<a
					href={sectionPath}
					aria-current={isActive ? "page" : undefined}
					class:text-zinc-50={isActive}
					class:text-zinc-400={!isActive}
					class:font-bold={isActive}>
					{section.title}
				</a>
			{/each}
		</div>

		{#if initialUpdateDone && navLinksContainer}
			<div
				id="nav-underline"
				class="absolute bottom-0 h-0.5 bg-white"
				style:left="{underlineLeft}px"
				style:width="{underlineWidth}px"
				role="presentation">
			</div>
		{/if}
	</div>

	<div class="px-10 pt-5">
		{@render children()}
	</div>
</div>

<style>
	#nav-underline {
		view-transition-name: nav-section-underline;
	}
</style>
