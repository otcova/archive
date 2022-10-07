import { invoke } from "@tauri-apps/api";
import { UtcDate } from "./date";

export async function fetch_done_commands_count_vs_days(from: UtcDate) {
	return await invoke("done_commands_count_vs_days", { from })
}