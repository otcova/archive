import { Button } from "../components/button";
import { ErrorLog } from "../database";
import style from "../styles/errorPanel.module.css"

type Props = {
	errorLog: ErrorLog,
}

export function ErrorPanel(props: Props) {
	return <div className={style.container} data-tauri-drag-region>
		<div className={style.panel}>
			{
				props.errorLog.errorMsg &&
				<div className={style.errorMsg}> {props.errorLog.errorMsg} </div>
			}
			{
				props.errorLog.msg &&
				<div> {props.errorLog.msg} </div>
			}
			{
				props.errorLog.button &&
				<Button txt={props.errorLog.button} action={props.errorLog.action} style={1} />
			}
		</div>
	</div >;
}

