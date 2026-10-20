import('./pkg/qwarks_www.js').then((qwarks) => qwarks.default())

// Returns a function that delays calling `fn` until `delay` ms have passed
// without another call (only the last call in a burst runs).
let debounce = (fn, delay) => {
    let timeout = null
    return (...args) => {
        clearTimeout(timeout)
        timeout = setTimeout(() => fn(...args), delay)
    }
}

// Handle resizing
let resize = (e) => {
    console.log('Resize event')
    let canvas = document.getElementById('spooky-canvas')
    canvas.width = canvas.clientWidth
    canvas.height = canvas.clientHeight
}
window.onresize = debounce(resize, 100)
resize()
