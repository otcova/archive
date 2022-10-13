import { render, Show } from 'solid-js/web'
import { databaseError } from './database'
import { saveDatabase } from './database/databaseState'
import './globalStyle/index.sass'
import ErrorPanel from './pages/ErrorPanel'
import { canLoadApp, showUpdatePanel, UpdatePanel } from './pages/UpdatePanel'
import TabSystem from './templates/TabSystem'
import WindowButtons from './templates/WindowButtons'

function App() {
    return <>
        <WindowButtons />
        {
            databaseError() ? <ErrorPanel /> :
                <>
                    <Show when={showUpdatePanel()}><UpdatePanel /> </Show>
                    <Show when={canLoadApp()}><TabSystem /></Show>
                </>
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

/*

- New Tab: `Ctrl T`
- Close Tab: `Ctrl W`
- Focus Tab: `Ctrl number`
- Focus Next Tab: `Ctrl Tab`
- Focus Previous Tab: `Ctrl Shift Tab`  
- Exit / Cancel / Revert changes: `Esc`
- Copy: `Ctrl C`
- Paste: `Ctrl V`
- Cut: `Ctrl X`
- Focus Next InputText: `Tab`
- Focus Previous InputText: `Shift Tab`
- Autocomplete Suggestion: `Enter`
- Choose Suggestion: `Up / Down`

*/