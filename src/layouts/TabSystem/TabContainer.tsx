import { useState } from "preact/hooks";
import { genId } from "../../utils/id";
import { clamp } from "../../utils/math";
import { useEvent } from "../../utils/useEvent";
import { Tab, TabTemplate } from "./tab";
import { TabBar } from "./TabBar";
import { TabContext } from "./tabContext";

type Props = { defaultTabTemplate: TabTemplate<any> }

export function TabContainer({ defaultTabTemplate }: Props) {
	const defaultTab: Tab<any> = { ...defaultTabTemplate, id: -1 };
	let [tabs, setTabs] = useState([defaultTab]);
	let [selected, select] = useState(defaultTab);

	const renameSelected = (title: string) => {
		const tab = { ...selected, title }
		const tabIndex = tabPos(selected)
		setTabs([...tabs.slice(0, tabIndex), tab, ...tabs.slice(tabIndex + 1, tabs.length)])
		select(tab)
	}

	const createTab = (tabTemplate: TabTemplate<any>) => {
		const tab = { ...tabTemplate, id: genId() }
		setTabs([...tabs, tab])
		select(tab)
	}

	const deleteTab = (tab: Tab<any>) => {
		let pos = tabPos(tab)
		if (pos < 0 || tabs.length < pos) return select(tabs[0])
		let newTabs = [...tabs.slice(0, pos), ...tabs.slice(pos + 1, tabs.length)]
		setTabs(newTabs)
		select(tabs[clamp(pos, 0, newTabs.length - 1)])
	}

	const tabPos = (tab: Tab<any>) => tabs.indexOf(tab)


	useEvent("keydown", event => {
		if (event.key == "Tab" && event.ctrlKey) {
			let tabIndex = tabPos(selected) + (event.shiftKey ? -1 : 1)
			select(tabs[clamp(tabIndex, 0, tabs.length - 1)])
		}
	});

	return <TabContext.Provider value={{ tabs, deleteTab, createTab, selected, select, renameSelected }}>
		<div className={"column exp"}>
			<TabBar />
			<div className="column exp" style={{ padding: "20px" }}>
				<selected.ContentClass {...(selected.props ?? {})} />
			</div>
		</div>
	</TabContext.Provider>
}

