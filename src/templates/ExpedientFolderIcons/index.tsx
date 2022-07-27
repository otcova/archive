import IconButton, { IconType } from "../../atoms/IconButton"
import { Expedient, folderOfExpedient } from "../../database/types"
import style from "./ExpedientFolderIcons.module.sass"
import { readDir, createDir } from '@tauri-apps/api/fs';
import { open } from '@tauri-apps/api/shell';
import { createSignal, onCleanup } from "solid-js";

type Props = {
	expedient: Expedient
}

export default function ExpedientFolderIcons(props: Props) {

	const [content, setContent] = createSignal([])

	const dir = () => folderOfExpedient(props.expedient)

	const readDirContent = async () => {
		try {
			setContent((await readDir(dir())).map(item => [item.path, item.children ? "folder" : "file"]))
		} catch (_) { }
	}

	readDirContent()
	let intervalId = setInterval(readDirContent, 1000)
	onCleanup(() => clearInterval(intervalId))

	const openPath = async path => {
		await createDir(path, { recursive: true })
		await open(path)
	}

	return <div class={style.container}>
		<IconButton icon='folder' action={() => openPath(dir())} />
		<div class={style.folder_space}></div>
		{content().map(([path, type]) => <PathIcon path={path} type={type} />)}
	</div>
}

function PathIcon({ path, type }: { path: string, type: "file" | "folder" }) {
	let icon: IconType = type == "folder" ? "folder" : "document"

	if (type == "file") {
		const name = path.split("/").pop().split("\\").pop().split(".")
		if (name.length <= 1) return
		const extension = name.pop()
		if (extension == "pdf") icon = "pdf"
		if (["png", "ico", "jpg", "gif"].includes(extension)) icon = "image"
	}

	return <IconButton icon={icon} action={() => open(path)} />
}