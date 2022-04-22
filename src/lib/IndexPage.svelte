<script lang="ts">
import { http, invoke } from "@tauri-apps/api";
import type { Modlist } from "./ReqTypes";
import { Data } from "./store";



	let metaData = "https://gist.githubusercontent.com/pozm/762df1b57718a53bc55a0ffa3dea47d2/raw/055603256d9c6bb24d0036f85b92dea3bb6ddfc1/pog.json";

	let mods = http.fetch<Modlist>(metaData)

</script>



<h1 class="text-gray-200 text-4xl" >Vulnus Mod Assistant</h1>
<p class="text-gray-400" >Babaooey!</p>
<h1 class="text-gray-300 text-3xl" >Mods</h1>
<div>
	{#await mods then resolvedMods}
		{#each resolvedMods.data.mods as mod}
			<div class="first:mt-0 my-4 bg-neutral-900 shadow-lg border-2 border-zinc-800 rounded-xl border-solid w-full h-11 flex flex-row align-middle items-center px-3" >
				<p class="text-gray-300 mr-auto" >{mod.name}</p>
				<button on:click={()=>{
					invoke("install_plugin",{
						pluginName:mod.name,
						remoteDir:mod.remote_url,
						vulnusDir:Data.Store.get.entries["Vulnus.path"]
					})
				}} class="mt-0 py-1 shadow-sm px-12 transition-colors hover:bg-indigo-600 text-gray-100 bg-indigo-500 disabled:bg-indigo-600/50 mt-2 rounded-lg">Install</button>
			</div>
		{/each}
	{/await}
</div>