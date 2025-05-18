<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import { scale } from "svelte/transition";
	import { writeText } from "@tauri-apps/plugin-clipboard-manager";
	import { open } from "@tauri-apps/plugin-shell";
	import { quintOut } from "svelte/easing";

	async function copyAndOpen() {
		await writeText(loginCode).then(() => open(verificationUri));
	}

	type Props = {
		loginCode: string;
		verificationUri: string;
		onCancel: () => void | Promise<void>;
	};

	let { loginCode, verificationUri, onCancel }: Props = $props();
</script>

<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900/50 backdrop-blur-sm transition-[backdrop-filter] duration-300"
	in:scale={{ duration: 300, start: 0.5, opacity: 0, easing: quintOut }}
	out:scale={{ duration: 200, opacity: 0, easing: quintOut }}>
	<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
		<Card.Header>
			<h2 class="text-lg font-bold text-zinc-50">Login Verification</h2>
			<p class="mt-2 font-bold text-zinc-50">Please enter the following code:</p>
		</Card.Header>
		<Card.Content>
			<p class="my-4 font-mono text-xl text-zinc-50">{loginCode}</p>
		</Card.Content>
		<Card.Footer class="flex justify-center gap-2">
			<Button onclick={copyAndOpen} variant="outline">Copy and Open</Button>
			<Button onclick={onCancel} variant="destructive">Cancel</Button>
		</Card.Footer>
	</Card.Root>
</div>
