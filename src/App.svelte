
<script lang="ts">
	import Modal from './lib/Modal.svelte';
import { dialog,fs,invoke } from "@tauri-apps/api";

import IndexPage from "./lib/IndexPage.svelte";
import SettingsPage from "./lib/SettingsPage.svelte";
import { Data } from "./lib/store";
import { ShowPathModal,ShowInstallModal, VulnusPath } from './lib/StoreData'
import {event} from '@tauri-apps/api'
import NotificationHandler from './lib/NotificationHandler.svelte';
import { onDestroy, onMount } from 'svelte';
import { checkBepinexInstall, installBepinex } from './lib/SharedFunctions';


	let updatePath = "";
	let PathActive = false;
	let awaitingData = Data.Store.get.reload();
	awaitingData.then(data=>{
		updatePath = data["Vulnus.path"]
		if ((data["Vulnus.path"] ?? '') == '') {
			ShowPathModal.set(true)
		} else {
			VulnusPath.set(data["Vulnus.path"]);
		}
	})

	function onModalSave() {
		Data.Store.get.add("Vulnus.path", updatePath);
		VulnusPath.set(updatePath);
		ShowPathModal.set(false)
		checkBepinexInstall(true)
		// ShowInstallModal.set(true);
	}
	function GetVulnusPath() {
		console.log("GetVulnusPath");
		dialog.open({
			directory:true,
			title:"Select Vulnus path",
			multiple:false
		}).then(v=>{
			updatePath = v as string;
		})
	}
	let PathIsInvalid = false;
	$: {
		invoke<boolean>("check_dir_exists",{direc: updatePath}).then(d=>{
			PathIsInvalid=!d;
			console.log(PathIsInvalid)
		})
	}
	let PagesMap = [IndexPage,SettingsPage]
	let ShowPage = 0;

	let interval;

	// onMount(()=>{
	// 	let bbb = 0;
	// 	interval = setInterval(()=>{
	// 		event.emit("client://notification",{title:"bruh",data:`aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaapog${bbb++}`})

	// 	},2e3)
	// })

	// onDestroy(()=>{
	// 	clearInterval(interval)
	// })

</script>
<div class="flex" >
	<NotificationHandler/>
	<Modal show={$ShowPathModal}>
		<h1 class="text-gray-200 text-2xl" >Hi,</h1>
		<h3 class="text-gray-300 text-lg">It seems like this is your first time using <span class="text-pink-300" >Vulnus Mod Assistant</span></h3>
		<div class="relative">
			<label for="password" class="block text-sm font-medium text-gray-400" >Please tell us where your install is located</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={updatePath} id="path" name="path" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border ${PathIsInvalid ? "border-red-400 ring-red-400" : "border-zinc-600"}`} placeholder="Path of vulnus">
			<button on:click={GetVulnusPath} class="absolute right-2 bottom-2.5 w-6 h-6" >
				<svg xmlns="http://www.w3.org/2000/svg" class={`h-6 w-6 ${PathActive ? "text-pink-200" : "text-zinc-600"}`} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
				</svg>
			</button>
		</div>
		<div class="flex justify-end" >
			{#if PathIsInvalid}
				<p class="mr-auto mt-2 text-sm text-red-400 select-none " >The path you have provided is is invalid </p>
				
			{/if}
			<button disabled={PathIsInvalid} class="py-2 shadow-sm px-12 transition-colors hover:bg-green-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg" on:click="{onModalSave}">Save</button>
		</div>
	</Modal>
	<Modal show={$ShowInstallModal} prematureClose={true} >
		<h1 class="text-gray-200 text-2xl" >Hi,</h1>
		<h3 class="text-gray-300 text-lg max-w-lg">It seems like you don't have bepinex installed, since it is required to use mods, would you like to install it?</h3>
		<div class="flex justify-end space-x-4" >

			<button class="py-2 shadow-sm px-12 transition-colors hover:bg-red-600 text-gray-100 bg-red-500 disabled:bg-red-600/50 mt-2 rounded-lg" on:click={()=>ShowInstallModal.set(false)}>No</button>
			<button class="py-2 shadow-sm px-12 transition-colors hover:bg-green-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg" on:click={()=>{
				installBepinex();
				ShowInstallModal.set(false);
			}}>Yes</button>
		</div>
	</Modal>
	<div class="min-h-screen w-20 bg-zinc-800 flex flex-col" >
		<div class="mx-auto py-2" on:click={()=>ShowPage=0} >
			<svg xmlns="http://www.w3.org/2000/svg" class="h-14 w-14 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
			</svg>
		</div>
		<div class="mx-auto py-2" on:click={()=>ShowPage=1} >
			<svg xmlns="http://www.w3.org/2000/svg" class="h-14 w-14 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
				<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
			  </svg>
		</div>
	</div>
	
	<div class="p-4 w-full" >
		<!-- <h1 class="text-gray-200 text-xl" >Vulnus Mod Assistant</h1> -->
		<svelte:component this={PagesMap[ShowPage]} />
	</div>
</div>