import { relaunch } from '@tauri-apps/api/process'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { createSignal, Show } from 'solid-js'
import Button from '../../atoms/Button'
import style from "./UpdatePanel.module.sass"

export const [showUpdatePanel, setShowUpdatePanel] = createSignal(true)

const [shouldUpdate, setShouldUpdate] = createSignal<boolean | string>(false)
checkUpdate().then(({ shouldUpdate, manifest }) => {
	if (shouldUpdate) setShouldUpdate(manifest.version)
	else setShouldUpdate(false)
	setShowUpdatePanel(shouldUpdate)
})

export function UpdatePanel() {
	return <div class={style.container} data-tauri-drag-region>
		<div class={style.panel}>
			<Show when={shouldUpdate()} fallback={"Buscant actualització..."}>
				{`Nova versió ${shouldUpdate()} !!!`}
				<div class={style.buttons}>
					<Button text='Cancelar' red keyMap='Escape' action={() => setShowUpdatePanel(false)} />
					<Button text='Actualitzar' keyMap='Enter' action={async () => {
						try {
							await installUpdate()
							await relaunch()
						} catch (error) {
							console.error(error)
						}
					}} />
				</div>
			</Show>
		</div>
	</div>
}