import { LuckyWheel } from 'lucky-wheel-wasm'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>Webassembly</h1>
    <div>
      <button id="play">Play</button>
    </div>
    <div class="canvas-container">
      <canvas id="canvas" width="400" height="400"></canvas>
      <div class="arrow">
        <div class="arrow-left"></div>
      </div>
    </div>
  </div>
`

const buttonEl = document.querySelector<HTMLButtonElement>('#play')
const luckyWheel = new LuckyWheel("canvas")

if (buttonEl) {
  buttonEl.addEventListener('click', () => {
    luckyWheel.start()
  })
}
