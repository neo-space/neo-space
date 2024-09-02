import init, { Renderer } from '../pkg/macro_cosmos.js';

async function run() {
    await init();
    const renderer = new Renderer();
    renderer.resize_canvas();
    
    let isDrawing = false;
    let startX, startY;

    const canvas = document.getElementById('canvas');
    canvas.style.position = 'absolute';
    canvas.style.top = '0';
    canvas.style.left = '0';
    canvas.style.pointerEvents = 'none';

    const previewCanvas = document.createElement('canvas');
    previewCanvas.style.position = 'absolute';
    previewCanvas.style.top = '0';
    previewCanvas.style.left = '0';
    previewCanvas.style.pointerEvents = 'none';
    document.body.appendChild(previewCanvas);

    function resizeCanvases() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        previewCanvas.width = window.innerWidth;
        previewCanvas.height = window.innerHeight;
        renderer.resize_canvas();
    }

    resizeCanvases();

    function getMousePos(clientX, clientY) {
        const rect = canvas.getBoundingClientRect();
        return {
            x: clientX - rect.left,
            y: clientY - rect.top
        };
    }

    function drawPreviewSquare(startX, startY, endX, endY) {
        const ctx = previewCanvas.getContext('2d');
        ctx.clearRect(0, 0, previewCanvas.width, previewCanvas.height);
        ctx.fillStyle = 'rgba(0, 0, 255, 0.2)';
        const width = endX - startX;
        const height = endY - startY;
        ctx.fillRect(
            Math.min(startX, endX),
            Math.min(startY, endY),
            Math.abs(width),
            Math.abs(height)
        );
    }

    document.addEventListener('mousedown', (e) => {
        isDrawing = true;
        const pos = getMousePos(e.clientX, e.clientY);
        startX = pos.x;
        startY = pos.y;
    });

    document.addEventListener('mousemove', (e) => {
        if (isDrawing) {
            const pos = getMousePos(e.clientX, e.clientY);
            drawPreviewSquare(startX, startY, pos.x, pos.y);
        }
    });

    document.addEventListener('mouseup', (e) => {
        if (!isDrawing) return;
        isDrawing = false;
        const pos = getMousePos(e.clientX, e.clientY);
        const width = pos.x - startX;
        const height = pos.y - startY;
        const r = Math.random();
        const g = Math.random();
        const b = Math.random();
        const a = Math.random();
        renderer.add_square(
            Math.min(startX, pos.x),
            Math.min(startY, pos.y),
            Math.abs(width),
            Math.abs(height),
            r, g, b, a
        );
        renderer.draw_all_shapes();
        
        // Clear the preview
        const ctx = previewCanvas.getContext('2d');
        ctx.clearRect(0, 0, previewCanvas.width, previewCanvas.height);
    });

    document.addEventListener('mouseleave', () => {
        isDrawing = false;
        // Clear the preview
        const ctx = previewCanvas.getContext('2d');
        ctx.clearRect(0, 0, previewCanvas.width, previewCanvas.height);
    });

    window.addEventListener('resize', () => {
        resizeCanvases();
        renderer.draw_all_shapes();
    });

    // Initial draw
    renderer.draw_all_shapes();
}

run();