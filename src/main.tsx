import "preact/debug";
import { render } from 'preact/compat'
import { App } from './app'
import './styles/index.css'

render(<App />, document.getElementById('app')!)


// Hide context menu
oncontextmenu = e => e.preventDefault()

// Update mouse frames
if (!document.getElementById("dummy")) {
    let dummy = document.createElement("div")
    dummy.id = "dummy"
    dummy.style.position = "absolute"
    document.body.appendChild(dummy)
    let draw = (t: number) => { dummy.style.top = t % 10 + "px"; requestAnimationFrame(draw) }
    draw(0)
}


// Refresh
addEventListener("keydown", event => {
    if (event.ctrlKey && event.key.toUpperCase() == "R")
        location.reload()
})