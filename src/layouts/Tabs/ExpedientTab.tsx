import { useContext } from "preact/hooks"
import { Button } from "../../components/button"
import { Tab } from "../TabSystem/tab"
import { TabContext } from "../TabSystem/tabContext"
import style from "../../styles/expedientTab.module.css"
import { BottomButtons } from "../BottomButtons"

type Props = {
	expedientId?: number,
}

export function ExpedientTab(props: Props) {
    let tabCtx = useContext(TabContext)

    const arxivar = () => {
        tabCtx.deleteTab(tabCtx.selected)
    }
    
    return <>
        <div className="expY">
            
        </div>
        <BottomButtons buttons={[
            { txt: "Arxivar", action: arxivar },
            { txt: "3 Similars", action: () => { } },
            { txt: "Desfer Canvis", action: () => { } },
        ]} />
    </>
}