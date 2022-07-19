import Button from "../../atoms/Button"
import { databaseError } from "../../database"
import style from "./ErrorPanel.module.sass"

export default function ErrorPanel() {
	return <div class={style.container}  data-tauri-drag-region>
		<div class={style.panel}>
			{
				databaseError().error &&
				<div class={style.error_text}>{databaseError().error}</div>
			}
			{databaseError().msg}
			{
				databaseError().button &&
				<Button text={databaseError().button} action={databaseError().action} />
			}
		</div>
	</div>
} 