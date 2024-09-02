import init, { Renderer } from '../pkg/macro_cosmos.js';

async function run() {
    await init();
    const renderer = new Renderer();
    
    let isDrawing = false;
    let isDragging = false;
    let startX, startY;
    let lastX, lastY;
    let userAction = 'drag'; // Default action

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const toolbar = document.getElementById('toolbar');
    const squareBtn = document.getElementById('squareBtn');
    const dragBtn = document.getElementById('dragBtn');

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

    function isOverToolbar(e) {
        const toolbarRect = toolbar.getBoundingClientRect();
        return (
            e.clientX >= toolbarRect.left &&
            e.clientX <= toolbarRect.right &&
            e.clientY >= toolbarRect.top &&
            e.clientY <= toolbarRect.bottom
        );
    }

    function updateCursor() {
        if (userAction === 'drag') {
            canvas.style.cursor = 'grab';
        } else {
            canvas.style.cursor = 'default';
        }
    }

    canvas.addEventListener('mousedown', (e) => {
        if (isOverToolbar(e)) return;
        
        const pos = getMousePos(e.clientX, e.clientY);
        startX = lastX = pos.x;
        startY = lastY = pos.y;

        if (userAction === 'square') {
            isDrawing = true;
        } else if (userAction === 'drag') {
            isDragging = true;
            canvas.style.cursor = 'grabbing';
        }
    });

    canvas.addEventListener('mousemove', (e) => {
        if (isOverToolbar(e)) return;

        const pos = getMousePos(e.clientX, e.clientY);

        if (isDrawing) {
            drawTranslucentSquare(startX, startY, pos.x, pos.y);
        } else if (isDragging) {
            const dx = pos.x - lastX;
            const dy = pos.y - lastY;
            // Implement dragging logic here (e.g., move the view)
            console.log(`Dragged by dx: ${dx}, dy: ${dy}`);
            lastX = pos.x;
            lastY = pos.y;
        }
    });

    canvas.addEventListener('mouseup', (e) => {
        if (isOverToolbar(e)) return;

        const pos = getMousePos(e.clientX, e.clientY);

        if (isDrawing) {
            isDrawing = false;
            renderer.add_square(startX, startY, pos.x, pos.y);
        } else if (isDragging) {
            isDragging = false;
            updateCursor();
        }
    });

    canvas.addEventListener('mouseleave', () => {
        if (isDrawing) {
            isDrawing = false;
            renderer.draw_all_squares();  // Clear the translucent square
        }
        if (isDragging) {
            isDragging = false;
            updateCursor();
        }
    });

    squareBtn.addEventListener('click', () => {
        userAction = 'square';
        updateCursor();
    });

    dragBtn.addEventListener('click', () => {
        userAction = 'drag';
        updateCursor();
    });

    window.addEventListener('resize', resizeCanvas);

    // Initial setup
    resizeCanvas();
    updateCursor();
}

run();