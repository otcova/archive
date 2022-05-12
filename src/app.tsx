import { TabBar } from "./components";
import { PageCotxe } from "./pages/cotxes";
import { PageExpedient } from "./pages/expedient";

export function App() {
    
    const tabs = [
        {name: "Cotxes", content: <PageCotxe/>},
        {name: "Xenia Freixes", content: <PageExpedient/>},
    ]
    
    return <>
        <TabBar tabs={tabs}/>
    </>
}