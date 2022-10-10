import { createSignal, Show } from "solid-js";
import LineChart from "../../atoms/LineChart";
import { utcDateNow } from "../../database/date";
import { fetch_done_commands_count_vs_days } from "../../database/statistics";
import style from "./Statistics.module.sass";


export default function Statistics() {
	const [graphData, setGraphData] = createSignal(null)
	fetch_done_commands_count_vs_days(utcDateNow()).then(d => setGraphData(d))

	return <div class={style.container}>
		<Show when={graphData()}>
			<LineChart
				title="Expedients tancats"
				data={acumulateTerms(graphData().slice(0, graphData().length))}
			/>
			<LineChart
				title="Expedients tancats per dia"
				data={graphData().slice(0, graphData().length)}
			/>
		</Show>
	</div>
}

function acumulateTerms(data: number[]): number[] {
	let newData = Array(data.length)
	if (data.length) {
		newData[0] = data[0]
		for (let i = 1; i < data.length; ++i) newData[i] = newData[i - 1] + data[i]
	}
	return newData
}