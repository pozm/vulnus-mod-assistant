import { event, invoke } from "@tauri-apps/api";
import { ShowInstallModal, VulnusPath } from "./StoreData";
import { get } from 'svelte/store';
import type { Mod } from "./ReqTypes";
import { Data } from "./store";


export function installBepinex() {
	invoke("install_bepinex",{vulnusDir:get(VulnusPath)}).then(_=>{
		event.emit("client://notification",{title:"Bepinex installed",data:`Enjoy modding!`})
	},e=>{
		console.log("BEPINEX FAILED TO INSTALL: ",e)
		event.emit("client://notification",{title:"Bepinex failed to install",data:`For some reason bepinex has failed installing. please contact me to get this fixed.`})

	})
}
export function installMod(mod:Mod) {

	checkBepinexInstall(true).then(success=>{

		if (success)
			invoke("install_plugin",{
				pluginName:mod.name,
				remoteDir:mod.remote_url,
				vulnusDir:Data.Store.get.entries["Vulnus.path"]
			}).then(_=>{
				event.emit("client://notification",{title:`${mod.name} has been installed!`,data:`Enjoy modding!`})
			},e=>{
				console.log("MOD FAILED TO INSTALL: ",e)
				event.emit("client://notification",{title:`${mod.name} failed to install`,data:`For some reason bepinex has failed installing. please contact me to get this fixed.`})
				
			})
		else {
			console.log("not installing mod cuz bepinex not installed.")
		}
	})
}
export function checkBepinexInstall(showModel=false) : Promise<boolean> {
	return new Promise((res,rej)=>{

		invoke<boolean>("check_install_bepinex",{vulnusDir:get(VulnusPath)}).then(v=>{
			if (!v && showModel) {
				ShowInstallModal.set(true)
			}
			res(v);
			// console.log(`${v} ${typeof v}`)
		},e=>{
			rej(e);
		})
	})
}