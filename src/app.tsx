import { TabContainer } from "./layouts/tab/TabContainer";
import { Ajenda } from "./layouts/ajenda";

export function App() {
    return <TabContainer defaultTabTemplate={{
        title: "Ajenda", ContentClass: Ajenda,
    }} />
}