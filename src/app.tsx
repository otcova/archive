import { PageCotxe } from "./pages/cotxes";
import { PageExpedient } from "./pages/expedient";

export function App() {
    return <PageExpedient />
}

oncontextmenu = e => e.preventDefault()

if (!document.getElementById("dummy")) {
    let dummy = document.createElement("div")
    dummy.id = "dummy"
    dummy.style.position = "absolute"
    document.body.appendChild(dummy)
    let draw = (t: number) => { dummy.style.top = t % 10 + "px"; requestAnimationFrame(draw) }
    draw(0)
}

addEventListener("keydown", event => {
    if (event.ctrlKey && event.key.toUpperCase() == "R")
        location.reload()
})