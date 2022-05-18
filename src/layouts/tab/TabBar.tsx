import { useContext } from "preact/hooks"
import { CloseWindow, MinimizeWindow } from "../../components/WindowButtons"
import style from "../styles/TabBar.module.css"
import { Tab } from "./tab";
import { TabContext } from "./tabContext";


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
	const tabCtx = useContext(TabContext);
	let isSelected = tabCtx.selected.id == tab.id;
	return <div
		className={isSelected ? style.tab : style.selectedTab}
		onClick={() => tabCtx.select(tab)}>
		{tab.title}
	</div>
}