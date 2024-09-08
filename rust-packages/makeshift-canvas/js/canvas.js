const canvas = new fabric.Canvas('canvas', {
    width: window.innerWidth,
    height: window.innerHeight,
    isDrawingMode: false
});

let isPanning = false;
let panOffsetX = 0;
let panOffsetY = 0;

// Set up infinite panning
canvas.on('mouse:down', function(opt) {
    const evt = opt.e;
    if (isPanning) {
        this.isDragging = true;
        this.selection = false;
        this.lastPosX = evt.clientX;
        this.lastPosY = evt.clientY;
    }
});

canvas.on('mouse:move', function(opt) {
    if (this.isDragging) {
        const e = opt.e;
        const vpt = this.viewportTransform;
        vpt[4] += e.clientX - this.lastPosX;
        vpt[5] += e.clientY - this.lastPosY;
        this.requestRenderAll();
        this.lastPosX = e.clientX;
        this.lastPosY = e.clientY;

        // Update dot grid position
        panOffsetX += e.clientX - this.lastPosX;
        panOffsetY += e.clientY - this.lastPosY;
        document.body.style.backgroundPosition = `${panOffsetX}px ${panOffsetY}px`;
    }
});

canvas.on('mouse:up', function(opt) {
    this.isDragging = false;
    this.selection = true;
});

// Zoom functionality
canvas.on('mouse:wheel', function(opt) {
    var delta = opt.e.deltaY;
    var zoom = canvas.getZoom();
    zoom *= 0.999 ** delta;
    if (zoom > 20) zoom = 20;
    if (zoom < 0.01) zoom = 0.01;
    canvas.zoomToPoint({ x: opt.e.offsetX, y: opt.e.offsetY }, zoom);
    opt.e.preventDefault();
    opt.e.stopPropagation();

    // Update dot grid size
    const gridSize = 20 * zoom;
    document.body.style.backgroundSize = `${gridSize}px ${gridSize}px`;
});

// Resize canvas on window resize
window.addEventListener('resize', () => {
    canvas.setDimensions({
        width: window.innerWidth,
        height: window.innerHeight
    });
});