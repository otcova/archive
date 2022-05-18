import { useEffect, useState } from "preact/hooks";
import { genId } from "../../utils/id";
import { clamp } from "../../utils/math";
import { useEvent } from "../../utils/useEvent";
import { Tab, TabTemplate } from "./tab";
import { TabBar } from "./TabBar";
import { TabContext } from "./tabContext";

type Props = { defaultTabTemplate: TabTemplate }

export function TabContainer({ defaultTabTemplate }: Props) {
	const defaultTab: Tab = { ...defaultTabTemplate, id: -1 };
	let [tabs, setTabs] = useState([defaultTab]);
	let [selected, select] = useState(defaultTab);

	const createTab = (tabTemplate: TabTemplate, selectTab = true) => {
		const tab = { ...tabTemplate, id: genId() }
		setTabs([...tabs, tab])
		if (selectTab) select(tab)
	}

	const deleteTab = (tab: Tab) => {
		console.log("TODO!");
	}

	const tabPos = (tab: Tab) => tabs.indexOf(tab)


	useEvent("keydown", event => {
		if (event.key == "Tab" && event.ctrlKey) {
			let tabIndex = tabPos(selected) + (event.shiftKey ? -1 : 1)
			select(tabs[clamp(tabIndex, 0, tabs.length - 1)])
		}
	});

	return <TabContext.Provider value={{ tabs, deleteTab, createTab, selected, select }}>
		<div className={"column"}>
			<TabBar />
			<selected.ContentClass />
		</div>
	</TabContext.Provider>
}

