import { batch, createContext, createSignal, For, useContext } from 'solid-js'
import { createStore } from 'solid-js/store'
import Agenda from '../Agenda'
import style from './TabSystem.module.sass'

const TabContext = createContext<{ focusTab, isActive, tab, createTab, closeTab, rename }>()
export const useTab = () => useContext(TabContext)

export default function TabSystem() {
	const initialTab = [{ name: "Agenda", componentClass: Agenda, props: {} }]
	const [tabs, setTabs] = createStore<{ name, componentClass, props, component?}[]>(initialTab)
	const [activeTab, setActiveTab] = createSignal(0)

	const tabsStore = tabIndex => ({
		focusTab: () => setActiveTab(tabIndex()),
		isActive: () => activeTab() == tabIndex(),
		tab: () => tabs[tabIndex()],
		rename: (newName: string) => setTabs(tabIndex(), "name", newName),
		createTab: (name, componentClass, props = {}) => {
			const index = tabIndex() + 1
			const newTab = { name, componentClass, props }
			batch(() => {
				setTabs(tabs => [...tabs.slice(0, index), newTab, ...tabs.slice(index, tabs.length)])
				setActiveTab(index)
			})
		},
		closeTab: () => {
			const index = tabIndex()
			batch(() => {
				if (index + 1 >= tabs.length) setActiveTab(index - 1)
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
	const { focusTab, isActive, tab } = useTab()
	return <div
		class={isActive() ? style.tab_active : style.tab}
		onClick={focusTab}>
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