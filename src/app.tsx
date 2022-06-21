import { TabContainer } from "./layouts/TabSystem/TabContainer"
import { AjendaTab } from "./layouts/Tabs/AjendaTab"
import { ErrorPanel } from "./layouts/ErrorPanel"
import { useState } from "preact/hooks"
import { currentErrorLog } from "./database"

export let updateScene = () => {}

export function App() {
    let [_, setSceneIndex] = useState(0)
    updateScene = () => setSceneIndex(i => i + 1)

    let errorLog = currentErrorLog()

    if (errorLog) return <ErrorPanel errorLog={errorLog} />
    
    return <TabContainer defaultTabTemplate={{
        title: "Ajenda", ContentClass: AjendaTab,
    }} />
}