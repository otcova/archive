import { TabContainer } from "./layouts/TabSystem/TabContainer";
import { AjendaTab } from "./layouts/Tabs/AjendaTab";

export function App() {
    return <TabContainer defaultTabTemplate={{
        title: "Ajenda", ContentClass: AjendaTab,
    }} />
}