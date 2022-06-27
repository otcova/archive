import { render } from 'solid-js/web'
import './index.sass'
import ErrorPanel from './templates/ErrorPanel'
import TabSystem from './templates/TabSystem'
import { databaseError } from './database'

function App() {
	return <>{
		databaseError() ? <ErrorPanel /> : <TabSystem />
	}</>
}

render(App, document.getElementById('root') as HTMLElement)

oncontextmenu = e => e.preventDefault()

// Update every frames to prevent bug
if (!document.getElementById("dummy")) {
    let dummy = document.createElement("div")
    dummy.id = "dummy"
    dummy.style.position = "absolute"
    document.body.appendChild(dummy)
    let draw = (t: number) => { dummy.style.top = t % 10 + "px"; requestAnimationFrame(draw) }
    draw(0)
}