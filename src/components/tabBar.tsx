import { JSX } from "preact";
import style from "./styles/tab-bar.module.css"
import { appWindow } from '@tauri-apps/api/window'
import { StateUpdater, useState } from "preact/hooks";

type Tab = { name: string, content: JSX.Element }

type Props = {
	tabs: Tab[]
}

export function TabBar({ tabs }: Props) {

	let [activeTab, setActiveTab] = useState(tabs[0]);

	return <div className={style.container}>
		<div data-tauri-drag-region className={style.tabs_container}>
			{tabs.map(tab => <Tab
				key={tab.name}
				active={activeTab.name == tab.name}
				activeTab={() => setActiveTab(tab)}
				tab={tab}
			/>)}
		</div>
		<div className={style.minimize} onClick={() => appWindow.minimize()}></div>
		<div className={style.close} onClick={() => appWindow.close()}></div>
	</div>
}

type TabProps = {
	tab: Tab,
	active: boolean,
	activeTab: () => void,
}

function Tab({ tab, active, activeTab }: TabProps) {
	return <div
		className={style.tab + (active ? " " + style.tab_active : "")}
		onClick={activeTab}>
		Cotxes
	</div>
}