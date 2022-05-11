import { JSX } from "preact";
import style from "./styles/tab-bar.module.css"
import { appWindow } from '@tauri-apps/api/window'

type Props = {}

export function TabBar(props: Props) {
	const tabs = [];

	tabs.push(<Tab key={0} active={true} />)
	tabs.push(<Tab key={1} active={false} />)

	return <div className={style.container}>
		<div data-tauri-drag-region className={style.tabs_container}>
			{tabs}
		</div>
		<div className={style.minimize} onClick={() => appWindow.minimize()}></div>
		<div className={style.close} onClick={() => appWindow.close()}></div>
	</div>
}

function Tab({ active }: { active: boolean }) {
	return <div className={style.tab + (active ? " " + style.tab_active : "")}>
		Cotxes
	</div>
}