<script lang="ts">
	import { onDestroy, onMount, type Snippet } from "svelte";
	import type { LayoutData } from "./$types";
	import type { Instance } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import Clock from "lucide-svelte/icons/clock";
	import CalendarClock from "lucide-svelte/icons/calendar-clock";
	import SquareChevronRight from "lucide-svelte/icons/square-chevron-right";
	import { Button } from "$lib/components/ui/button";
	import InstanceAssetsDownloadPopUp from "$lib/components/core/InstanceAssetsDownloadPopUp.svelte";
	import { formatDistanceToNow, parseISO } from "date-fns";
	import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	let instance = $state<Instance>();
	let isLaunching = $state(false);

	let unlistenLaunchStarted: UnlistenFn | undefined;
	let unlistenLaunchFinished: UnlistenFn | undefined;

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
		if (isLaunching) return;

		try {
			await invoke("launch_instance", { slug: data.slug });
		} catch (error) {
			console.error("Failed to launch instance:", error);
		} finally {
			await getInstance();
		}
	}

	async function getInstance() {
		await invoke<Instance>("get_instance", { slug: data.slug }).then((data) => {
			instance = data;
		});
	}

	async function openLogWindow() {
		console.log("Opening log window");

		const label = `${instance?.slug}-log`.replaceAll(".", "_");
		console.log("Log window label:", label);

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

	onMount(async () => {
		await getInstance();

		const label = `${instance?.slug}`.replaceAll(".", "_");
		const launchStartedEvent = `${label}-launch-started`;
		const launchFinishedEvent = `${label}-launch-finished`;

		try {
			unlistenLaunchStarted = await listen(launchStartedEvent, (event) => {
				console.log("Launch started event received:", event);
				isLaunching = true;
			});

			unlistenLaunchFinished = await listen(launchFinishedEvent, (event) => {
				console.log("Launch finished event received:", event);
				isLaunching = false;
				getInstance();
			});
		} catch (e) {
			console.error("Error setting up event listeners:", e);
		}
	});

	onDestroy(() => {
		unlistenLaunchStarted?.();
		unlistenLaunchFinished?.();
	});
</script>

<InstanceAssetsDownloadPopUp />

<div class="w-full overflow-hidden font-display">
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
		<Button class="w-30 px-10" onclick={launchInstance} disabled={isLaunching}>
			{#if isLaunching}
				<span class="animate-pulse">Launching...</span>
			{:else}
				Launch
			{/if}
		</Button>
		<Button variant="outline" class="w-24 px-10" onclick={openLogWindow}>
			<SquareChevronRight /> Logs
		</Button>
	</div>

	<div class="mt-5 flex space-x-10 px-10 text-xl font-bold">
		{#each data.sections as section}
			<a href="/#/launcher/instance/{data.slug}/{section.slug}">{section.title}</a>
		{/each}
	</div>

	<div class="px-10">
		{@render children()}
	</div>
</div>
