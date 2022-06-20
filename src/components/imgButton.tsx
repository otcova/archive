
import style from "../styles/buttons.module.css"
import { useEvent } from "../utils/useEvent"

type Props = {
	img: number,
	shortCut?: string, // ctrl a, alt r, ctrl alt n
	action: () => any,
}

export function ImgButton(props: Props) {

	useEvent("keydown", event => {
		if (!props.shortCut) return
		let cmd = props.shortCut.toLocaleUpperCase().trim()
		let cmdKey = cmd.split(/ +/g).pop();
		if (cmd.includes("CTRL") && !event.ctrlKey) return
		if (cmd.includes("ALT") && !event.altKey) return
		if (event.code == "Key" + cmdKey) props.action()
	})

	return <div
		className={"pointer"}
		onClick={props.action}>
		{images[props.img]}
	</div>
	// return images[0];
}

let images = [
	<svg
		width="24.29563"
		height="19.750858"
		viewBox="0 0 24.29563 19.750858"><path
			id="path136094"
			style="fill:none;fill-opacity:1;stroke:#5c5c5c;stroke-width:1.5;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1;stop-color:#000000"
			d="m 1.665155,1.0025139 h 9.34635 l 1.58006,3.19719 10.03896,1.2e-4 c 0.36851,0 0.66509,0.28817 0.66509,0.6457 V 18.105134 c 0,0.35776 -0.29669,0.64571 -0.66509,0.64571 H 1.665155 c -0.36852,0 -0.66514,-0.2878 -0.66514,-0.64571 V 1.9135939 c 0,-0.35738 0.29859,-0.94708005 0.66514,-0.91108 z" />
	</svg>, <svg
		width="20"
		height="20"
		viewBox="0 0 20 20">
		<rect
			style="fill:none;fill-opacity:1;stroke:#5c5c5c;stroke-width:1.5;stroke-linecap:round;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1;paint-order:normal;stop-color:#000000"
			width="15"
			height="15"
			x="1"
			y="4.5"
			rx="5"
			ry="5" />
		<path
			style="fill:none;stroke:#ffffff;stroke-width:10;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
			d="M 8.5,11.5 18,2" />
		<path
			style="fill:none;stroke:#5c5c5c;stroke-width:1.5;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
			d="m 8.5,11.5 10,-10 0.0611407,7" />
		<path
			style="fill:none;stroke:#5c5c5c;stroke-width:1.5;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
			d="m 18.5,1.5 h -7" />
	</svg>
];