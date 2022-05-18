import { createContext } from "preact";
import { StateUpdater } from "preact/hooks";
import { Tab, TabTemplate } from "./tab";

type TabContextType = {
	tabs: Tab[],
	deleteTab: (tab: Tab) => void,
	createTab: (template: TabTemplate, select: boolean) => void,
	selected: Tab,
	select: StateUpdater<Tab>,
}

export const TabContext = createContext<TabContextType>(undefined!);