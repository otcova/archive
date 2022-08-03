import { render } from 'solid-js/web'
import { databaseError } from './database'
import { saveDatabase } from './database/databaseState'
import './globalStyle/index.sass'
import ErrorPanel from './pages/ErrorPanel'
import TabSystem from './templates/TabSystem'
import WindowButtons from './templates/WindowButtons'

function App() {
    return <>
        <WindowButtons />
        {
            databaseError() ? <ErrorPanel /> : <TabSystem />
        }
    </>
}

render(App, document.getElementById('root') as HTMLElement)

oncontextmenu = event => event.preventDefault()

// prevent middle click scroll
document.body.onmousedown = event => {
    if (event.button == 1) event.preventDefault()
}

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

addEventListener("keydown", event => {
    // Save on reload
    if (event.ctrlKey && event.code == "KeyR")
        saveDatabase()

    // Blur (unfocus) all when Esc
    if (event.code == "Escape")
        document.activeElement["blur"]?.()
})
