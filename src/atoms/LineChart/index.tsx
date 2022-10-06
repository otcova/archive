import { onMount } from "solid-js";

type Props = {
	data: number[],
	acumulate?: boolean,
	title: string,
};

const WIDTH = 600, HEIGHT = 330;

export default function LineChart(props: Props) {
	let canvas: HTMLCanvasElement
	onMount(() => {
		const global_max = props.acumulate ? 
			props.data.reduce((a, b) => a + b) : Math.max(...props.data)

		const c = canvas.getContext("2d")
		c.clearRect(0, 0, canvas.width, canvas.height)
		c.font = '16px Arial'

		const marginX = 50, marginY = 50
		const x0 = marginX, y0 = canvas.height - marginY
		const x1 = canvas.width - marginX, y1 = marginY

		c.lineWidth = 1

		const x_divisions = 5
		const y_divisions = 5
		const division_width = (x1 - x0) / x_divisions
		const division_height = (y1 - y0) / y_divisions

		const y_division_scale = Math.round(global_max / y_divisions) + 1

		c.strokeStyle = "#999"

		c.textBaseline = "middle"
		c.textAlign = "right"
		for (let y = 0; y <= y_divisions; ++y) {
			const offset = division_height * y
			line(c, x0, y0 + offset, x1, y0 + offset)
			const value = y_division_scale * y
			if (value != 0) c.fillText(value + "", x0 - 8, y0 + offset)
		}
		
		c.textBaseline = "top"
		c.textAlign = "center"
		for (let x = 0; x <= x_divisions; ++x) {
			const offset = division_width * x
			line(c, x0 + offset, y0, x0 + offset, y1)
			const value = Math.round((x_divisions - x) * props.data.length / (x_divisions))
			c.fillText(value + "", x0 + offset, y0 + 10)
		}

		////// Draw Line

		const y_scale = (y1 - y0) / (y_divisions * y_division_scale)
		const x_scale = (x1 - x0) / props.data.length

		c.strokeStyle = "#06F"
		c.lineWidth = 2
		c.lineJoin = "bevel"
		c.beginPath()
		let value = props.data[props.data.length - 1]
		c.moveTo(x0, y0 + value * y_scale)
		for (let x = props.data.length - 1; x >= 0; --x) {
			if (props.acumulate) value += props.data[x]
			else value = props.data[x]
			c.lineTo(x1 - x * x_scale, y0 + value * y_scale)
		}
		c.stroke()
		
		// Draw Headers
		const xHalf = canvas.width / 2;
		const yHalf = canvas.width / 2;

		c.fillText(props.title, xHalf, 15)
		// c.fillText(props.title, xHalf, 5)
	})

	return <canvas width={WIDTH + "px"} height={HEIGHT + "px"} ref={canvas}></canvas>
}

function line(context: CanvasRenderingContext2D, ax: number, ay: number, bx: number, by: number) {
	context.beginPath()
	context.moveTo(ax + .5, ay + .5)
	context.lineTo(bx + .5, by + .5)
	context.stroke()
}

