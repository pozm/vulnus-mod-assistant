<script lang="ts">
	import { invoke } from '@tauri-apps/api'
import { installBepinex } from './SharedFunctions';
import { Data } from './store';
import { ShowPathModal, VulnusPath } from './StoreData';

</script>
<div class="space-y-2" >

	<h1 class="text-gray-200 text-5xl mb-4" >Settings</h1>
	<div class="my-4 bg-neutral-900 p-2 border border-solid border-neutral-700 rounded-xl shadow-lg" >

		<div class="flex flex-row w-full content-center items-center" >
			<p class="text-gray-300 text-xl mr-auto" >Vulnus Path: <span class="text-pink-300" >{$VulnusPath}</span></p>
			<button on:click={()=>{ShowPathModal.set(true)}} class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-cyan-600 text-gray-100 bg-cyan-500 disabled:bg-cyan-600/50 mt-2 rounded-lg">Change</button>
		</div>
		<div class="flex flex-row w-full content-center items-center" >
			<p class="text-gray-300 text-xl mr-auto" >Install Bepinex</p>
			<button on:click={installBepinex} class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-indigo-600 text-gray-100 bg-indigo-500 disabled:bg-indigo-600/50 mt-2 rounded-lg">Install</button>
		</div>
	</div>
</div>