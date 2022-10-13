import { onMount } from "solid-js";

type Props = {
	data: number[], // [oldest, ..., newest]
	title: string,
};

const WIDTH = 600, HEIGHT = 350;

export default function LineChart(props: Props) {
	let canvas: HTMLCanvasElement

	let zoom = 0

	onMount(() => {
		const c = canvas.getContext("2d")
		let transition_start = 0
		const transition_ms = 400

		const transition = () => {
			if (canvas.isConnected) {
				transition_start ||= performance.now()
				const x = Math.min(1, (performance.now() - transition_start) / transition_ms)
				const scale = Math.sin((x * Math.PI) / 2);
				const data = props.data
				drawChart(canvas, c, props.title, data, scale, zoom)
				requestAnimationFrame(transition);
			}
		}
		requestAnimationFrame(transition);
	})

	const onWheel = (event: WheelEvent) => {
		if (event.ctrlKey) {
			const change = (.05 * event.deltaY / 100) / (2 * zoom + .5)
			const frames = 10
			for (let i = 0; i < frames; ++i) {
				setTimeout(() => {
					zoom -= change / frames
					zoom = Math.max(0, Math.min(0.95, zoom))
				}, i * 16)
			}
		}
	}

	return <canvas width={WIDTH + "px"} height={HEIGHT + "px"} ref={canvas} onWheel={onWheel}></canvas>
}

function drawChart(
	canvas: HTMLCanvasElement,
	c: CanvasRenderingContext2D,
	title: string,
	data: number[],
	animation_scale: number,
	zoom: number) {

	function setStyle(style: { [k in keyof CanvasRenderingContext2D]?: CanvasRenderingContext2D[k] }) {
		for (const styleName in style) c[styleName] = style[styleName]
	}

	function line(ax: number, ay: number, bx: number, by: number) {
		c.beginPath()
		c.moveTo(ax + .5, ay + .5)
		c.lineTo(bx + .5, by + .5)
		c.stroke()
	}

	const global_max = Math.max(...data)

	c.clearRect(0, 0, canvas.width, canvas.height)
	const fontFamily = " roboto, Arial"
	c.font = '13px' + fontFamily

	const marginX = 50, marginY = 70
	const x0 = marginX, y0 = canvas.height - marginY
	const x1 = canvas.width - marginX, y1 = marginY

	const x_divisions = 4
	const y_divisions = 5
	const division_width = (x1 - x0) / x_divisions
	const division_height = (y1 - y0) / y_divisions

	const y_division_scale = Math.round(global_max / y_divisions) + 1

	setStyle({ lineWidth: 1, strokeStyle: "#aaa" })
	setStyle({ textAlign: "right", textBaseline: "middle", })

	for (let y = 0; y <= y_divisions; ++y) {
		const offset = division_height * y
		line(x0, y0 + offset, x1, y0 + offset)
		const value = y_division_scale * y
		if (value != 0) c.fillText(value + "", x0 - 8, y0 + offset)
	}

	let divisions = divide_data(data, zoom)
	// Draw Data Divisions
	setStyle({ textAlign: "center", textBaseline: "top" })
	let x = x0
	for (let div_index = 0; div_index < divisions.length; ++div_index) {
		const div = divisions[div_index]
		line(x, y0, x, y1)
		c.fillText(div.tag, x + div.size * (x1 - x0) / 2, y0 + 5)
		x += div.size * (x1 - x0)
	}
	line(x1, y0, x1, y1)

	// Draw Data Line
	setStyle({ fillStyle: "#06f6", strokeStyle: "#06f", lineWidth: 1.5, lineJoin: "bevel" })
	c.beginPath()
	let firstX = x0
	x = x0
	for (let div_index = 0; div_index < divisions.length; ++div_index) {
		const div = divisions[div_index]
		for (let i = 0; i < div.data.length; ++i) {
			const y = y0 + animation_scale * div.data[i] * (y1 - y0) / global_max
			if (div.data[i] !== null) {
				if (div_index == 0 && div.data[i - 1] == null) {
					c.moveTo(x, y0)
					c.moveTo(x, y)
					firstX = x
				}
				else c.lineTo(x, y)
				if (div_index == divisions.length - 1 && div.data[i + 1] == null) {
					c.stroke()
					c.lineTo(x, y0)
				}
			}
			x += div.size * (x1 - x0) * 1 / div.data.length
		}
	}
	c.lineTo(firstX, y0)
	c.fill()

	// Draw Headers
	const xHalf = canvas.width / 2;
	const yHalf = canvas.width / 2;

	setStyle({ fillStyle: "#000" })
	setStyle({ font: "16px" + fontFamily, textBaseline: "bottom", textAlign: "center" })
	c.fillText(title, xHalf, marginY - 15)
}


const monthsNames = [
	"Gen", "Febr", "Març", "Abr", "Maig", "Juny", "Jul", "Ag", "Set", "Oct", "Nov", "Des",
]

type DataDivison = {
	data: number[],
	tag: string,
	size: number,
}
function divide_data(data: number[], zoom: number): DataDivison[] {
	const divisions: DataDivison[] = [];

	const now = new Date()
	let date = new Date(now.getTime())
	let zoomed_date = new Date(now.getTime())
	date.setDate(date.getDate() - data.length + 1)
	zoomed_date.setDate(zoomed_date.getDate() - Math.round(data.length * (1 - zoom)) + 1)

	const months = 12 * (now.getFullYear() - zoomed_date.getFullYear()) + now.getMonth() - zoomed_date.getMonth()

	if (months > 7) {
		let index = 0;
		while (date.getFullYear() <= now.getFullYear()) {
			let year = date.getFullYear()

			const millis = new Date(year + 1, 0, 1).getTime() - date.getTime()
			const days = Math.round(millis / (1000 * 60 * 60 * 24))

			const data_slice = data.slice(index, index + days)
			data_slice.push(...Array(days - data_slice.length).fill(null))
			index += days

			divisions.push({
				data: data_slice,
				size: 0,
				tag: year + "",
			})
			date = new Date(year + 1, 0, 1)
		}
	} else {

		let index = 0;
		while (date.getFullYear() < now.getFullYear() || date.getMonth() <= now.getMonth()) {
			const lastDateOfMonth = new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate()

			const days = lastDateOfMonth - date.getDate() + 1
			const data_slice = data.slice(index, index + days)
			index += days
			if (index >= data.length) {
				data_slice.push(...Array(index - data.length).fill(null))
			}


			divisions.push({
				data: data_slice,
				size: 0,
				tag: monthsNames[date.getMonth()]
			})
			date = new Date(date.getFullYear(), date.getMonth() + 1, 1)
		}
	}

	// Normalize size
	const total_size = divisions.reduce((sum, div) => sum + div.data.length, 0)
	for (const div of divisions) div.size = div.data.length / total_size;

	// Zoom (Cut data)
	date = new Date(now.getTime())
	date.setDate(date.getDate() - data.length + 1)

	while (date < zoomed_date) {
		let after_delete_date = new Date(date.getTime())
		after_delete_date.setDate(after_delete_date.getDate() + divisions[0].data.length)

		if (after_delete_date < zoomed_date) {
			divisions.shift()
			date = after_delete_date;
		} else {
			const days = (zoomed_date.getTime() - date.getTime()) / (1000 * 60 * 60 * 24)
			divisions[0].data.splice(0, Math.floor(days))
			break;
		}
	}

	// Normalize size
	const final_size = divisions.reduce((sum, div) => sum + div.data.length, 0)
	for (const div of divisions) div.size = div.data.length / final_size;


	// Remove small tags
	for (const div of divisions) {
		if (div.size < 0.01) div.tag = ""
	}

	return divisions
}