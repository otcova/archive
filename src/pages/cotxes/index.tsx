import { useState } from "preact/compat";
import { CotxesBar, Filter } from "./cotxes-bar";
import { CotxesBody } from "./cotxes-body";

const defaultFilter: Filter = { checked: false }

export function PageCotxe() {
	
	const [filter, setFilter] = useState(defaultFilter);
	
	return <>
		<CotxesBar filter={filter} onFilterChange={setFilter} />
		<CotxesBody filter={filter} />
	</>
}
