<script lang="ts">
	import { Label } from "$lib/components/ui/label";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import { platform } from "@tauri-apps/plugin-os";
	import { open } from "@tauri-apps/plugin-dialog";
	import { onMount } from "svelte";
	import { saveJavaToConfig, type ManualJava } from "$lib/utils/JavaUtils";
	import { commands, type JavaTestInfo } from "$lib/bindings";

	let manualJava8 = $state<ManualJava>({ version: 8, path: "" });
	let manualJava17 = $state<ManualJava>({ version: 17, path: "" });
	let manualJava21 = $state<ManualJava>({ version: 21, path: "" });

	let javaTestResults = $state<JavaTestInfo[]>([]);
	let showManualJavaTestPopup = $state(false);

	onMount(async () => {
		await getJavaFromConfig();

		if (prefilledPaths) {
			if (prefilledPaths.java8) manualJava8.path = prefilledPaths.java8;
			if (prefilledPaths.java17) manualJava17.path = prefilledPaths.java17;
			if (prefilledPaths.java21) manualJava21.path = prefilledPaths.java21;
		}
	});

	async function getJavaFromConfig() {
		await commands.getJavaFromConfig().then((res) => {
			if (res.status === "ok") {
				let { java8Path, java17Path, java21Path } = res.data;

				if (!prefilledPaths?.java8) manualJava8.path = java8Path;
				if (!prefilledPaths?.java17) manualJava17.path = java17Path;
				if (!prefilledPaths?.java21) manualJava21.path = java21Path;
			} else {
				console.error("Failed to get Java from config:", res.error);
			}
		});
	}

	async function testJava() {
		if (!manualJava8.path || !manualJava17.path || !manualJava21.path) {
			return;
		}

		await commands.testJava([manualJava8.path, manualJava17.path, manualJava21.path]).then((res) => {
			if (res.status === "ok") {
				javaTestResults = res.data.map((item) => ({
					...item,
					version: item.version
				}));
				showManualJavaTestPopup = true;
			} else {
				console.error("Failed to test Java:", res.error);
			}
		});
	}

	type Props = {
		onComplete?: () => void;
		prefilledPaths?: {
			java8: string | null;
			java17: string | null;
			java21: string | null;
		};
	};

	let { onComplete, prefilledPaths }: Props = $props();
</script>

{#if showManualJavaTestPopup}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
		<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
			<Card.Header>
				<h2 class="text-xl font-bold text-zinc-50">Test Java Setup</h2>
			</Card.Header>
			<Card.Content>
				{#each javaTestResults as result, index}
					<div class="mb-4 rounded-md border border-zinc-800 p-3">
						<h3 class="mb-2 font-semibold text-zinc-50">
							Java {[8, 17, 21][index]}
						</h3>
						{#if result.valid && !result.versionMismatch}
							<div class="space-y-1 text-sm">
								<p class="text-green-500">✅ Java detected</p>
								<p class="text-zinc-300">Version: {result.version}</p>
								<p class="text-zinc-300">Vendor: {result.vendor}</p>
							</div>
						{:else if result.versionMismatch}
							<div class="space-y-1 text-sm">
								<p class="text-yellow-500">
									⚠️ Warning: Expected Java {result.expectedVersion} but found Java {result.version}
								</p>
								<p class="text-zinc-300">Vendor: {result.vendor}</p>
							</div>
						{:else}
							<p class="text-red-500">❌ Java not found or invalid</p>
						{/if}
					</div>
				{/each}
			</Card.Content>
			<Card.Footer class="flex justify-center gap-4">
				<Button
					onclick={() => {
						showManualJavaTestPopup = false;
						saveJavaToConfig([manualJava8.path, manualJava17.path, manualJava21.path], false);
						onComplete?.();
					}}>
					Save
				</Button>
				<Button variant="destructive" onclick={() => (showManualJavaTestPopup = false)}>Cancel</Button>
			</Card.Footer>
		</Card.Root>
	</div>
{/if}

<div>
	{@render javaPathInput(manualJava8)}
	{@render javaPathInput(manualJava17)}
	{@render javaPathInput(manualJava21)}
	<Button class="mt-4" variant="secondary" onclick={testJava}>Test</Button>
</div>

{#snippet javaPathInput(obj: { version: number; path: string })}
	<div class="mt-4">
		<Label class="mb-2 block text-zinc-50">Java {obj.version} Path</Label>
		<div class="flex flex-row">
			<Input class="w-96" type="text" placeholder="/usr/lib/jvm" bind:value={obj.path} />
			<Button
				class="ml-2 h-8"
				variant="outline"
				onclick={async (e) => {
					e.preventDefault();

					let isWindows = platform() === "windows";
					let javaPath = await open({
						title: "Select Java",
						multiple: false,
						filters: isWindows ? [{ name: "Java", extensions: ["exe"] }] : []
					});

					if (javaPath) {
						obj.path = javaPath;
					}
				}}>
				...
			</Button>
		</div>
	</div>
{/snippet}
