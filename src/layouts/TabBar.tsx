import { useContext } from "preact/hooks"
import { CloseWindow } from "../components/CloseWindow"
import { MinimizeWindow } from "../components/MinimizeWindow"
import style from "../styles/TabBar.module.css"
import { Tab, TabContext } from "./TabContainer"


export function TabBar() {
	const tabCtx = useContext(TabContext);
	
	return <div className={style.container}>
		<div className={style.tabsContainer}>
			{
				tabCtx.tabs.map(tab => <TabElement key={tab.id} tab={tab} />)
			}
		</div>
		<MinimizeWindow />
		<CloseWindow />
	</div>
}

function TabElement({ tab }: { tab: Tab }) {
	return <div className={style.tabElement}>{tab.title}</div>
}