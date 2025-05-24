<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { MoveLeft, MoveRight } from "@lucide/svelte";
	import JavaDownloadPopUp from "$lib/components/core/JavaDownloadPopUp.svelte";
	import ManualJavaSetup from "$lib/components/core/ManualJavaSetup.svelte";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
	import { onMount } from "svelte";
	import { commands, type JavaTestInfo } from "$lib/bindings";

	let detectedJavaPaths = $state<{
		java8: string | null;
		java17: string | null;
		java21: string | null;
	}>({ java8: null, java17: null, java21: null });
	let javaValidationResults = $state<JavaTestInfo[]>([]);
	let isDetectionComplete = $state(false);
	let isValidationComplete = $state(false);
	let shouldShowValidationResults = $state(false);

	let missingJavaVersions = $derived.by(() => {
		const missing = [];
		if (!detectedJavaPaths.java8) missing.push(8);
		if (!detectedJavaPaths.java17) missing.push(17);
		if (!detectedJavaPaths.java21) missing.push(21);
		return missing;
	});

	onMount(async () => {
		await detectInstalledJava();

		if (detectedJavaPaths.java8 || detectedJavaPaths.java17 || detectedJavaPaths.java21) {
			await validateJavaInstallations();
			shouldShowValidationResults = true;
		}
	});

	async function detectInstalledJava() {
		isDetectionComplete = false;

		commands.detectJava().then((res) => {
			if (res.status === "ok") {
				const [java8, java17, java21] = res.data;
				detectedJavaPaths = {
					java8: java8?.toString() || null,
					java17: java17?.toString() || null,
					java21: java21?.toString() || null
				};
			} else {
				console.error("Failed to detect Java:", res.error);
				detectedJavaPaths = { java8: null, java17: null, java21: null };
			}
			isDetectionComplete = true;
		});
	}

	async function validateJavaInstallations() {
		isValidationComplete = false;
		javaValidationResults = [];

		const pathsToValidate: [string, string, string] = [
			detectedJavaPaths.java8 || "",
			detectedJavaPaths.java17 || "",
			detectedJavaPaths.java21 || ""
		];

		if (!pathsToValidate.some((path) => path !== "")) {
			console.log("No Java installations provided to validate");
			isValidationComplete = true;
			return;
		}

		console.log("Validating Java installations with paths:", pathsToValidate);

		commands.testJava(pathsToValidate).then((res) => {
			if (res.status === "ok") {
				javaValidationResults = res.data;
				console.log("Java validation results:", javaValidationResults);
			} else {
				console.error("Failed to validate Java:", res.error);
			}
			isValidationComplete = true;
		});
	}

	async function handleDownloadComplete(finalJavaPathsFromDownload: [string, string, string]) {
		detectedJavaPaths = {
			java8: finalJavaPathsFromDownload[0] || null,
			java17: finalJavaPathsFromDownload[1] || null,
			java21: finalJavaPathsFromDownload[2] || null
		};
		isDetectionComplete = true;

		isValidationComplete = false;
		shouldShowValidationResults = false;
		javaValidationResults = [];

		await validateJavaInstallations();
		shouldShowValidationResults = true;

		onboardingController.handleAutomaticJavaSetupComplete();
	}

	function handleUseDetectedJava() {
		if (missingJavaVersions.length === 0) {
			const paths: [string, string, string] = [detectedJavaPaths.java8 || "", detectedJavaPaths.java17 || "", detectedJavaPaths.java21 || ""];

			commands.saveJavaToConfig(paths, false).then((res) => {
				if (res.status === "ok") {
					handleDownloadComplete(paths);
				} else {
					console.error("Failed to save Java paths:", res.error);
				}
			});
		} else {
			onboardingController.handleAutomaticJavaSetupClick();
		}
	}

	function getValidationStatus(version: number): { valid: boolean; vendor?: string; versionMismatch?: boolean } {
		if (!javaValidationResults.length) return { valid: false };

		const versionIndex = version === 8 ? 0 : version === 17 ? 1 : 2;
		const result = javaValidationResults[versionIndex];

		return {
			valid: result?.valid || false,
			vendor: result?.vendor,
			versionMismatch: result?.versionMismatch
		};
	}
</script>

{#if onboardingController.showAutomaticJavaPopUp}
	<JavaDownloadPopUp missingVersions={missingJavaVersions} detectedJava={detectedJavaPaths} onComplete={handleDownloadComplete} />
{/if}

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Set up Java</p>
	<p class="mt-4 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Either choose to set up Java automatically or manually.</p>

	{#if isDetectionComplete}
		<div class="mt-6 animate-fade-in rounded-lg border border-zinc-700 bg-zinc-900 p-4 opacity-0 [animation-delay:600ms]">
			<h3 class="mb-3 font-semibold text-zinc-50">Java Detection Results:</h3>
			<div class="space-y-2 text-sm">
				<div class="flex items-center justify-between">
					<span class="text-zinc-300">Java 8:</span>
					<span class={detectedJavaPaths.java8 ? "text-green-400" : "text-red-400"}>
						{detectedJavaPaths.java8 ? "✓ Detected" : "✗ Not found"}
					</span>
				</div>
				<div class="flex items-center justify-between">
					<span class="text-zinc-300">Java 17:</span>
					<span class={detectedJavaPaths.java17 ? "text-green-400" : "text-red-400"}>
						{detectedJavaPaths.java17 ? "✓ Detected" : "✗ Not found"}
					</span>
				</div>
				<div class="flex items-center justify-between">
					<span class="text-zinc-300">Java 21:</span>
					<span class={detectedJavaPaths.java21 ? "text-green-400" : "text-red-400"}>
						{detectedJavaPaths.java21 ? "✓ Detected" : "✗ Not found"}
					</span>
				</div>
			</div>
			{#if missingJavaVersions.length > 0 && !shouldShowValidationResults}
				<p class="mt-2 text-xs text-yellow-400">
					Missing Java versions will be downloaded: {missingJavaVersions.join(", ")}
				</p>
			{/if}
		</div>
	{/if}

	{#if shouldShowValidationResults && isValidationComplete}
		<div class="mt-4 animate-fade-in rounded-lg border border-green-700 bg-green-900/20 p-4 opacity-0 [animation-delay:200ms]">
			<h3 class="mb-3 font-semibold text-green-400">✓ Installation Verification:</h3>
			<div class="space-y-3 text-sm">
				{#each [8, 17, 21] as version}
					{@const hasJava =
						(version === 8 && detectedJavaPaths.java8) ||
						(version === 17 && detectedJavaPaths.java17) ||
						(version === 21 && detectedJavaPaths.java21)}
					{#if hasJava}
						{@const status = getValidationStatus(version)}
						<div class="rounded-md border border-zinc-800 bg-zinc-900/50 p-3">
							<div class="mb-1 flex items-center justify-between">
								<span class="font-medium text-zinc-300">Java {version}:</span>
								<span class={status.valid ? "text-green-400" : "text-red-400"}>
									{status.valid ? "✓ Working" : "✗ Failed"}
								</span>
							</div>
							{#if status.valid && status.vendor}
								<p class="text-xs text-zinc-400">Vendor: {status.vendor}</p>
								{#if status.versionMismatch}
									<p class="text-xs text-yellow-400">⚠️ Version mismatch detected</p>
								{/if}
							{/if}
						</div>
					{/if}
				{/each}
			</div>
		</div>
	{/if}

	<div class="mt-8 flex animate-fade-in flex-row gap-4 opacity-0 [animation-delay:1000ms]">
		<Button onclick={handleUseDetectedJava} disabled={!isDetectionComplete}>
			{missingJavaVersions.length === 0 ? "Use Detected Java" : "Download Missing Java"}
		</Button>
		<Button variant="destructive" onclick={() => onboardingController.handleManualJavaSetupClick()}>Manual Setup</Button>
	</div>

	{#if onboardingController.showManualJavaEntries}
		<p class="mt-8 text-lg">Enter the paths to your Java installations.</p>
		<div class="mt-4">
			<ManualJavaSetup prefilledPaths={detectedJavaPaths} onComplete={() => onboardingController.handleManualJavaSetupComplete()} />
		</div>
	{/if}

	<Button
		onclick={() => onboardingController.navigateToPrevious()}
		class="fixed bottom-4 left-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="mr-2 animate-bounce-left" />
		Back
	</Button>

	<div class="fixed bottom-4 right-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<Button
			onclick={() => onboardingController.navigateToNext()}
			disabled={onboardingController.isNextDisabled()}
			class="transition-opacity duration-200 {onboardingController.isNextDisabled() ? 'opacity-50' : 'opacity-100'}">
			Continue
			<MoveRight class="ml-2 animate-bounce-right" />
		</Button>
	</div>
</div>
