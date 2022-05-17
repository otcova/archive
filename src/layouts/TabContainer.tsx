import { createContext } from "preact";
import { StateUpdater, useState } from "preact/hooks";
import { Ajenda } from "./ajenda";
import { TabBar } from "./TabBar";

export interface Tab {
	title: string,
	id: number
	contentClass: () => JSX.Element,
}

const defaultTabs = [
	{
		title: "Ajenda",
		id: 0,
		contentClass: Ajenda,
	}
]

function renderTab(tab: Tab): JSX.Element {
	return <tab.contentClass key={tab.id} />
}

type TabContextType = {
	tabs: Tab[],
	setTabs: StateUpdater<Tab[]>,
	selected: number,
	select: StateUpdater<number>,
}

export const TabContext = createContext<TabContextType>(undefined!);

export function TabContainer() {
	let [tabs, setTabs] = useState(defaultTabs);
	let [selected, select] = useState(0);

	return <TabContext.Provider value={{ tabs, setTabs, selected, select }}>
		<div className={"column"}>
			<TabBar />
			{renderTab(tabs[selected])}
		</div>
	</TabContext.Provider>
}

