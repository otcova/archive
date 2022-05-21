import { useContext } from "preact/hooks"
import { Button } from "../../components/button"
import { ExpedientTab } from "./ExpedientTab"
import { TabContext } from "../TabSystem/tabContext"

export function AjendaTab() {
    let tabCtx = useContext(TabContext)

    const openBlankExpedient = () => {
        tabCtx.createTab({
            title: "Expedient",
            ContentClass: ExpedientTab,
        })
    }

    return <>
        <div className="expY">

        </div>
        <div className="row gap expX">
            <Button txt="Historial" action={() => { }} />
            <Button txt="Usuaris" action={() => { }} />
            <div className="expX"></div>
            <Button txt="Obrir Expedient" important action={openBlankExpedient} />
        </div>
    </>
}