import init, { Renderer } from '../pkg/macro_cosmos.js';

async function run() {
    await init();
    const renderer = new Renderer();
    
    let isDrawing = false;
    let startX, startY;

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        renderer.resize_canvas(window);
        renderer.draw_all_squares();
    }

    function getMousePos(clientX, clientY) {
        const rect = canvas.getBoundingClientRect();
        return {
            x: clientX - rect.left,
            y: clientY - rect.top
        };
    }

    function drawTranslucentSquare(startX, startY, endX, endY) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        renderer.draw_all_squares();
        
        ctx.fillStyle = 'rgba(0, 0, 255, 0.2)';  // Translucent blue
        ctx.fillRect(
            Math.min(startX, endX),
            Math.min(startY, endY),
            Math.abs(endX - startX),
            Math.abs(endY - startY)
        );
    }

    canvas.addEventListener('mousedown', (e) => {
        isDrawing = true;
        const pos = getMousePos(e.clientX, e.clientY);
        startX = pos.x;
        startY = pos.y;
    });

    canvas.addEventListener('mousemove', (e) => {
        if (!isDrawing) return;
        const pos = getMousePos(e.clientX, e.clientY);
        drawTranslucentSquare(startX, startY, pos.x, pos.y);
    });

    canvas.addEventListener('mouseup', (e) => {
        if (!isDrawing) return;
        isDrawing = false;
        const pos = getMousePos(e.clientX, e.clientY);
        renderer.add_square(startX, startY, pos.x, pos.y);
    });

    canvas.addEventListener('mouseleave', () => {
        if (isDrawing) {
            isDrawing = false;
            renderer.draw_all_squares();  // Clear the translucent square
        }
    });

    window.addEventListener('resize', resizeCanvas);

    // Initial setup
    resizeCanvas();
}

run();