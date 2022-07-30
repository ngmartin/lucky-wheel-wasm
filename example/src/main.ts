import { LuckyWheel } from 'lucky-wheel-wasm'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>Webassembly</h1>
    <canvas id="canvas" width="400" height="400"></canvas>
  </div>
`

new LuckyWheel("canvas").start()
