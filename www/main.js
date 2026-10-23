import { debounce } from 'lodash-es'
import initQwarks, * as qwarks from './pkg/qwarks_www.js'
await initQwarks()

// Extra supersampling applied on top of devicePixelRatio
const SUPERSAMPLE = 2

// Handle resizing: match every showcase canvas's buffer to its displayed size,
// scaled up by the device pixel ratio (crisp on HiDPI screens) and SUPERSAMPLE.
// The ratio is read on each resize since it changes with zoom or moving monitors.
let resize = (e) => {
    console.log('Resize event')
    let pixelScale = (window.devicePixelRatio || 1) * SUPERSAMPLE
    for (let canvas of document.querySelectorAll('.showcase-canvas')) {
        canvas.width = Math.round(canvas.clientWidth * pixelScale)
        canvas.height = Math.round(canvas.clientHeight * pixelScale)
    }
}
window.onresize = debounce(resize, 100)
resize()
