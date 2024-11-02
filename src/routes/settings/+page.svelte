<script lang="ts">
</script>

<div class="overflow-hidden bg-zinc-400 font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">Settings</p>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<label class="inline-flex items-center">
				<input type="checkbox" class="form-checkbox text-zinc-600" />
				<span class="ml-2 text-zinc-50">Start maximized</span>
			</label>
			<div class="mt-4">
				<label for="window-width" class="block text-zinc-50">Window Width</label>
				<input id="window-width" type="number" class="form-input mt-1 block w-full text-zinc-900" placeholder="Enter width" />
			</div>
			<div class="mt-4">
				<label for="window-height" class="block text-zinc-50">Window Height</label>
				<input id="window-height" type="number" class="form-input mt-1 block w-full text-zinc-900" placeholder="Enter height" />
			</div>
			<div class="mt-4">
				<label class="inline-flex items-center">
					<input type="checkbox" class="form-checkbox text-zinc-600" />
					<span class="ml-2 text-zinc-50">Use discrete GPU</span>
				</label>
			</div>
		</div>
	</div>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Java</h2>
		<div class="mt-4">
			<label for="java-path" class="block text-zinc-50">Java Path</label>
			<input id="java-path" type="text" class="form-input mt-1 block w-full text-zinc-900" placeholder="Enter Java path" />
		</div>
		<div class="mt-4">
			<label for="java-args" class="block text-zinc-50">Java Arguments</label>
			<input id="java-args" type="text" class="form-input mt-1 block w-full text-zinc-900" placeholder="Enter Java arguments" />
		</div>
	</div>



</div>
