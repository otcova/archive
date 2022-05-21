import { createContext } from "preact";
import { StateUpdater } from "preact/hooks";
import { Tab, TabTemplate } from "./tab";

type TabContextType = {
	tabs: Tab<any>[],
	deleteTab: (tab: Tab<any>) => void,
	createTab: (template: TabTemplate<any>) => void,
	selected: Tab<any>,
	select: StateUpdater<Tab<any>>,
}

export const TabContext = createContext<TabContextType>(undefined!);