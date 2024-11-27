<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import type { Instance } from "$lib/types";
	import type { PageData } from "../$types";
	import { invoke } from "@tauri-apps/api/core";
	import { goto } from "$app/navigation";

	let { data }: { data: PageData } = $props();
	let startMaximised = $state(false);
	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let instance = $state<Instance>();

	function getInstance() {
		invoke<Instance>("get_instance", { slug: data.slug }).then((data) => {
			instance = data;
		});
	}

	function deleteInstance() {
		console.log("Deleting instance " + data.slug);
		invoke("delete_instance", { slug: data.slug }).then(() => {
			console.log("Instance deleted");
			goto("/");
		});
	}


</script>

<div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<div class="inline-flex items-center">
				<Checkbox bind:checked={startMaximised} />
				<Label class="ml-2 text-zinc-50">Start maximised</Label>
			</div>
		</div>
	</div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Advanced</h2>
		<Button class="mt-2" variant="destructive" onclick={deleteInstance}>Delete Instance</Button>
	</div>
</div>
