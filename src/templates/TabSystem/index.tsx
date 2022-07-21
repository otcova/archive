import { batch, createContext, createSignal, For, useContext } from 'solid-js'
import { createStore } from 'solid-js/store'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'
import WindowButtons from '../../atoms/WindowButtons'
import Agenda from '../../pages/Agenda'
import ExpedientEditor from '../../pages/ExpedientEditor'
import style from './TabSystem.module.sass'

const TabContext = createContext<{ focusTab, isActive, tab, createTab, closeTab, rename }>()
export const useTab = () => useContext(TabContext)

export default function TabSystem() {
	const staticTabs = [
		// { name: "Nou Expedient", componentClass: ExpedientEditor, props: {} },
		{ name: <StaticCheckbox state={"Todo"} />, componentClass: Agenda, props: {} },
		{ name: <StaticCheckbox state={"Pending"} />, componentClass: Agenda, props: {} },
		{ name: <StaticCheckbox state={"Done"} />, componentClass: Agenda, props: {} },
	]

	const [tabs, setTabs] = createStore<{ name, componentClass, props, component?}[]>([...staticTabs])
	const [activeTab, setActiveTab] = createSignal(0)

	const tabsStore = tabIndex => ({
		focusTab: () => setActiveTab(tabIndex()),
		isActive: () => activeTab() == tabIndex(),
		tab: () => tabs[tabIndex()],
		rename: (newName: string) => setTabs(tabIndex(), "name", newName),
		createTab: (name, componentClass, props = {}) => {
			const index = Math.max(staticTabs.length, tabIndex() + 1)
			const newTab = { name, componentClass, props }
			batch(() => {
				setTabs(tabs => [...tabs.slice(0, index), newTab, ...tabs.slice(index, tabs.length)])
				setActiveTab(index)
			})
		},
		closeTab: () => {
			const index = tabIndex()
			if (staticTabs.length > index) return
			batch(() => {
				// Change selected tab
				if (index + 1 >= tabs.length) {
					// Default first static tab
					if (index == 3) setActiveTab(0)
					else setActiveTab(index - 1)
				}
				setTabs(tabs => [...tabs.slice(0, index), ...tabs.slice(index + 1, tabs.length)])
			})
		}
	})

	return (
		<>
			<div class={style.tab_bar} data-tauri-drag-region>
				<For each={tabs}>{(_, index) =>
					<TabContext.Provider value={tabsStore(index)}>
						<TabTitle />
					</TabContext.Provider>
				}</For>
			</div >
			<div class={style.tab_content}>
				<For each={tabs}>{(_, index) =>
					<TabContext.Provider value={tabsStore(index)}>
						<TabContent />
					</TabContext.Provider>
				}</For>
			</div>
		</>
	)
};

function TabTitle() {
	const { focusTab, isActive, tab, closeTab } = useTab()
	
	const onClick = (event: MouseEvent) => {
		if (event.button == 1) closeTab()
		else focusTab()
	}
	
	return <div
		class={isActive() ? style.tab_active : style.tab}
		onMouseUp={onClick}>
		{tab().name}
	</div>
}

function TabContent() {
	const { isActive, tab } = useTab()
	const component = tab().componentClass(tab().props)
	return <>{
		isActive() ? component : <></>
	}</>
}