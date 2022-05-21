import { useContext } from "preact/hooks"
import { Button } from "../../components/button"
import { Tab } from "../TabSystem/tab"
import { TabContext } from "../TabSystem/tabContext"

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
        <div className="row gap expX">
            <Button txt="3 Similars"  action={() => {}}/>
            <Button txt="Desfer Canvis"  action={() => {}}/>
            <div className="expX"></div>
            <Button txt="Arxivar" important action={arxivar}/>
        </div>
    </>
}