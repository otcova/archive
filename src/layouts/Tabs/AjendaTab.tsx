import { useContext } from "preact/hooks"
import { Button } from "../../components/button"
import { ExpedientTab } from "./ExpedientTab"
import { ExpedientList } from "../ExpedientList"
import { TabContext } from "../TabSystem/tabContext"
import style from "../../styles/expedientTab.module.css"
import { BottomButtons } from "../BottomButtons"

export function AjendaTab() {
    let tabCtx = useContext(TabContext)

    const openBlankExpedient = () => {
        tabCtx.createTab({
            title: "Expedient",
            ContentClass: ExpedientTab,
        })
    }

    return <>
        <ExpedientList />
        <BottomButtons buttons={[
            { txt: "Obrir Expedient", action: openBlankExpedient },
            { txt: "Historial", action: () => { } },
            { txt: "Usuaris", action: () => { } },
        ]} />
    </>
}