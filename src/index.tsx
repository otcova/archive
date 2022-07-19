import { render } from 'solid-js/web'
import './index.sass'
import ErrorPanel from './pages/ErrorPanel'
import TabSystem from './templates/TabSystem'
import { databaseError } from './database'
import WindowButtons from './atoms/WindowButtons'

function App() {
    return <>
        <WindowButtons />
        {
            databaseError() ? <ErrorPanel /> : <TabSystem />
        }
    </>
}

render(App, document.getElementById('root') as HTMLElement)

oncontextmenu = e => e.preventDefault()

// Update every frame to prevent input delay bug
if (!document.getElementById("dummy")) {
    let dummy = document.createElement("div")
    dummy.id = "dummy"
    dummy.style.position = "absolute"
    document.body.appendChild(dummy)
    let pos = 0
    let draw = () => {
        dummy.style.top = (pos = 1 - pos) + "px"
        requestAnimationFrame(draw)
    }
    draw()
}