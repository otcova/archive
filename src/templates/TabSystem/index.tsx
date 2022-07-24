import { batch, createContext, createSignal, For, JSX, useContext } from 'solid-js'
import Button from '../../atoms/Button'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'
import IconButton from '../../atoms/IconButton'
import { createExpedient } from '../../database/expedientState'
import { expedientUtils } from '../../database/types'
import DoneList from '../../pages/DoneList'
import ExpedientEditor from '../../pages/ExpedientEditor'
import OpenList from '../../pages/OpenList'
import PendingList from '../../pages/PendingList'
import style from './TabSystem.module.sass'

type Tab = {
	name,
	componentClass,
	props,
	component?,
}

const TabContext = createContext<{
	focusTab: () => void,
	isActive: () => boolean,
	tab: () => Tab,
	createTab: <T>(name: string, componentClass: (props: T) => JSX.Element, props: T) => void,
	closeTab: () => void,
	rename: (newName: string) => void
}>()

export const useTab = () => useContext(TabContext)

export default function TabSystem() {
	const staticTabs = [
		{ name: <StaticCheckbox state={"Todo"} />, componentClass: OpenList, props: {} },
		{ name: <StaticCheckbox state={"Pending"} />, componentClass: PendingList, props: {} },
		{ name: <StaticCheckbox state={"Done"} />, componentClass: DoneList, props: {} },
	]

	const [tabs, setTabs] = createSignal<Tab[]>([...staticTabs])
	const [activeTab, setActiveTab] = createSignal(0)

	const tabsContext = tabIndex => ({
		focusTab: () => setActiveTab(tabIndex()),
		isActive: () => activeTab() == tabIndex(),
		tab: () => tabs()[tabIndex()],
		rename: (newName: string) => setTabs(tabs => {
			tabs[tabIndex()].name = newName
			return [...tabs]
		}),
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

	const createExpedientOnNewTab = async () => {
		const { createTab } = tabsContext(() => tabs().length - 1)
		createTab("Nou Expedient", ExpedientEditor, {
			expedientId: await createExpedient(expedientUtils.newBlank()),
		})
	}

	return (
		<>
			<div class={style.tab_bar} data-tauri-drag-region>
				<For each={tabs()}>{(_, index) =>
					<TabContext.Provider value={tabsContext(index)}>
						<TabTitle />
					</TabContext.Provider>
				}</For>
				<IconButton icon='create' action={createExpedientOnNewTab} />
			</div >
			<div class={style.tab_content}>
				<For each={tabs()}>{(_, index) =>
					<TabContext.Provider value={tabsContext(index)}>
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