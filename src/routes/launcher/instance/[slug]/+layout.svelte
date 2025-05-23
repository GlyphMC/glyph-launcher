<script lang="ts">
	import { onDestroy, onMount, tick, type Snippet } from "svelte";
	import type { LayoutData } from "./$types";
	import { Clock, CalendarClock, SquareChevronRight, X } from "@lucide/svelte";
	import { Button } from "$lib/components/ui/button";
	import InstanceAssetsDownloadPopUp from "$lib/components/core/InstanceAssetsDownloadPopUp.svelte";
	import { formatDistanceToNow, parseISO } from "date-fns";
	import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
	import { type UnlistenFn } from "@tauri-apps/api/event";
	import type { Attachment } from "svelte/attachments";
	import { page } from "$app/state";
	import { scale } from "svelte/transition";
	import { quintOut } from "svelte/easing";
	import { commands, events, type Instance } from "$lib/bindings";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	let instance = $state<Instance>();
	let isInstanceRunning = $state(false);

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
			document.startViewTransition(() => updateStyles());
		} else {
			updateStyles();
		}
	}

	async function forceUpdateUnderline() {
		if (!initialUpdateDone || !navLinksContainer) return;

		await tick();
		updateUnderlinePosition(false);
	}

	onMount(async () => {
		await getInstance();
		await tick();

		updateUnderlinePosition(false);
		initialUpdateDone = true;
		window.addEventListener("force-underline-update", forceUpdateUnderline);
	});

	onDestroy(() => window.removeEventListener("force-underline-update", forceUpdateUnderline));

	$effect.pre(() => {
		if (navLinksContainer && initialUpdateDone) {
			page.url.hash; // Trigger update on hash change
			tick().then(() => updateUnderlinePosition(true));
		}
	});

	function formatTimePlayed(totalSeconds: number): string {
		if (!totalSeconds) return "Never";
		if (totalSeconds < 60) return `${totalSeconds} sec played`;

		const minutes = Math.floor(totalSeconds / 60);
		const hours = Math.floor(minutes / 60);
		const remainingMinutes = minutes % 60;

		return hours > 0 ? `${hours} hr ${remainingMinutes} min played` : `${minutes} min played`;
	}

	function formatLastPlayed(isoTimestamp?: string | null): string {
		if (!isoTimestamp) return "Never";
		try {
			return formatDistanceToNow(parseISO(isoTimestamp), { addSuffix: true });
		} catch {
			return "Invalid date";
		}
	}

	async function launchInstance() {
		await commands
			.launchInstance(data.slug)
			.then((res) => {
				if (res.status === "ok") {
					console.log("Instance launched successfully");
				} else {
					console.error("Failed to launch instance:", res.error);
				}
			})
			.then(async () => await getInstance());
	}

	async function killInstance() {
		if (!isInstanceRunning) return;
		await commands.killInstance(data.slug).then((res) => {
			if (res.status === "ok") {
				console.log("Instance killed successfully");
			} else {
				console.error("Failed to kill instance:", res.error);
			}
		});
	}

	async function getInstance() {
		await commands.getInstance(data.slug).then((res) => {
			if (res.status === "ok") {
				instance = res.data;
			} else {
				console.error("Failed to get instance:", res.error);
			}
		});
	}

	async function openLogWindow() {
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
			unlistenFns.push(
				await events.instanceStartedEvent.listen((event) => {
					console.log("Instance started event received");
					isInstanceRunning = true;
				})
			);

			unlistenFns.push(
				await events.instanceStoppedEvent.listen((event) => {
					console.log("Instance stopped event received");
					isInstanceRunning = false;
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
			<p>Time played: {formatTimePlayed(Number(instance?.settings?.timePlayed) || 0)}</p>
		</div>

		<div class="mt-1 flex items-center">
			<CalendarClock class="mr-2 size-4" />
			<p>Last played: {formatLastPlayed(instance?.settings?.lastPlayed)}</p>
		</div>
	</div>

	<div class="ml-10 mt-8 flex space-x-2">
		{#if isInstanceRunning}
			<Button class="w-30 bg-red-600 px-10 hover:bg-red-700" onclick={killInstance}>
				<X class="mr-2 size-4" /> Stop
			</Button>
		{:else}
			<Button class="w-30 px-10" onclick={launchInstance} disabled={isInstanceRunning}>Launch</Button>
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

	{#key page.url.hash}
		<div class="px-10 pt-5" in:scale={{ start: 0.95, duration: 250, easing: quintOut }}>
			{@render children()}
		</div>
	{/key}
</div>

<style>
	#nav-underline {
		view-transition-name: nav-section-underline;
	}
</style>
