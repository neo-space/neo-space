import init, { Renderer } from '../pkg/macro_cosmos.js';

async function run() {
    await init();
    const renderer = new Renderer();
    
    let isDrawing = false;
    let isDragging = false;
    let isEditing = false;
    let isSelecting = false;
    let startX, startY;
    let lastX, lastY;
    let userAction = 'drag'; // Default action
    let selectedSquares = [];
    let editingSquare = null;
    let resizeHandle = null;

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const toolbar = document.getElementById('toolbar');
    const squareBtn = document.getElementById('squareBtn');
    const dragBtn = document.getElementById('dragBtn');
    const editBtn = document.getElementById('editBtn');

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        renderer.resize_canvas(window);
        renderer.draw_all_squares();
        drawEditOverlay();
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
        drawEditOverlay();
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
        } else if (userAction === 'edit') {
            canvas.style.cursor = 'default';
        } else {
            canvas.style.cursor = 'default';
        }
    }

    function drawEditOverlay() {
        if (userAction === 'edit') {
            selectedSquares.forEach(square => {
                // Draw translucent fill
                ctx.fillStyle = 'rgba(173, 216, 230, 0.3)'; // Light blue with 30% opacity
                ctx.fillRect(square.start_x, square.start_y, square.width, square.height);
    
                // Draw light blue outline
                ctx.strokeStyle = 'rgb(173, 216, 230)'; // Light blue
                ctx.lineWidth = 2;
                ctx.strokeRect(square.start_x, square.start_y, square.width, square.height);
    
                drawResizeHandles(square);
            });
    
            if (isSelecting) {
                // Draw translucent fill for selection box
                ctx.fillStyle = 'rgba(173, 216, 230, 0.3)'; // Light blue with 30% opacity
                ctx.fillRect(
                    Math.min(startX, lastX),
                    Math.min(startY, lastY),
                    Math.abs(lastX - startX),
                    Math.abs(lastY - startY)
                );
    
                // Draw light blue outline for selection box
                ctx.strokeStyle = 'rgb(173, 216, 230)'; // Light blue
                ctx.lineWidth = 1;
                ctx.strokeRect(
                    Math.min(startX, lastX),
                    Math.min(startY, lastY),
                    Math.abs(lastX - startX),
                    Math.abs(lastY - startY)
                );
            }
        }
    }

    function drawResizeHandles(square) {
        const handleSize = 8;
        const handles = [
            { x: square.start_x, y: square.start_y },
            { x: square.start_x + square.width, y: square.start_y },
            { x: square.start_x, y: square.start_y + square.height },
            { x: square.start_x + square.width, y: square.start_y + square.height },
            { x: square.start_x + square.width / 2, y: square.start_y },
            { x: square.start_x + square.width, y: square.start_y + square.height / 2 },
            { x: square.start_x + square.width / 2, y: square.start_y + square.height },
            { x: square.start_x, y: square.start_y + square.height / 2 }
        ];

        ctx.fillStyle = 'white';
        handles.forEach(handle => {
            ctx.fillRect(handle.x - handleSize / 2, handle.y - handleSize / 2, handleSize, handleSize);
        });
    }

    function getClickedSquare(x, y) {
        // This function needs to be implemented in Rust
        // For now, return null
        return null;
    }

    function getResizeHandle(square, x, y) {
        const handleSize = 8;
        const handles = [
            { x: square.start_x, y: square.start_y, cursor: 'nwse-resize' },
            { x: square.start_x + square.width, y: square.start_y, cursor: 'nesw-resize' },
            { x: square.start_x, y: square.start_y + square.height, cursor: 'nesw-resize' },
            { x: square.start_x + square.width, y: square.start_y + square.height, cursor: 'nwse-resize' },
            { x: square.start_x + square.width / 2, y: square.start_y, cursor: 'ns-resize' },
            { x: square.start_x + square.width, y: square.start_y + square.height / 2, cursor: 'ew-resize' },
            { x: square.start_x + square.width / 2, y: square.start_y + square.height, cursor: 'ns-resize' },
            { x: square.start_x, y: square.start_y + square.height / 2, cursor: 'ew-resize' }
        ];

        for (let i = 0; i < handles.length; i++) {
            const handle = handles[i];
            if (Math.abs(x - handle.x) <= handleSize / 2 && Math.abs(y - handle.y) <= handleSize / 2) {
                return { index: i, cursor: handle.cursor };
            }
        }
        return null;
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
        } else if (userAction === 'edit') {
            const clickedSquare = getClickedSquare(pos.x, pos.y);
            if (clickedSquare) {
                editingSquare = clickedSquare;
                selectedSquares = [editingSquare];
                resizeHandle = getResizeHandle(editingSquare, pos.x, pos.y);
                if (resizeHandle) {
                    isEditing = true;
                    canvas.style.cursor = resizeHandle.cursor;
                }
            } else {
                isSelecting = true;
                selectedSquares = [];
            }
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
        } else if (isEditing && editingSquare && resizeHandle) {
            resizeSquare(editingSquare, resizeHandle.index, pos.x, pos.y);
            renderer.update_square(editingSquare);
            renderer.draw_all_squares();
            drawEditOverlay();
        } else if (isSelecting) {
            lastX = pos.x;
            lastY = pos.y;
            renderer.draw_all_squares();
            drawEditOverlay();
        } else if (userAction === 'edit') {
            const clickedSquare = getClickedSquare(pos.x, pos.y);
            if (clickedSquare) {
                const handle = getResizeHandle(clickedSquare, pos.x, pos.y);
                if (handle) {
                    canvas.style.cursor = handle.cursor;
                } else {
                    canvas.style.cursor = 'move';
                }
            } else {
                canvas.style.cursor = 'default';
            }
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
        } else if (isEditing) {
            isEditing = false;
            resizeHandle = null;
            updateCursor();
        } else if (isSelecting) {
            isSelecting = false;
            selectedSquares = renderer.get_squares_in_area(
                Math.min(startX, lastX),
                Math.min(startY, lastY),
                Math.abs(lastX - startX),
                Math.abs(lastY - startY)
            );
        }

        renderer.draw_all_squares();
        drawEditOverlay();
    });

    canvas.addEventListener('mouseleave', () => {
        if (isDrawing) {
            isDrawing = false;
            renderer.draw_all_squares();
        }
        if (isDragging) {
            isDragging = false;
            updateCursor();
        }
        if (isEditing) {
            isEditing = false;
            resizeHandle = null;
            updateCursor();
        }
        if (isSelecting) {
            isSelecting = false;
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

    editBtn.addEventListener('click', () => {
        userAction = 'edit';
        updateCursor();
    });

    window.addEventListener('resize', resizeCanvas);

    window.addEventListener('keydown', (e) => {
        if (e.key === 'Backspace' && userAction === 'edit' && selectedSquares.length > 0) {
            selectedSquares.forEach(square => {
                renderer.remove_square(square);
            });
            selectedSquares = [];
            renderer.draw_all_squares();
            drawEditOverlay();
        }
    });

    function resizeSquare(square, handleIndex, x, y) {
        switch (handleIndex) {
            case 0: // Top-left
                square.width += square.start_x - x;
                square.height += square.start_y - y;
                square.start_x = x;
                square.start_y = y;
                break;
            case 1: // Top-right
                square.width = x - square.start_x;
                square.height += square.start_y - y;
                square.start_y = y;
                break;
            case 2: // Bottom-left
                square.width += square.start_x - x;
                square.height = y - square.start_y;
                square.start_x = x;
                break;
            case 3: // Bottom-right
                square.width = x - square.start_x;
                square.height = y - square.start_y;
                break;
            case 4: // Top-middle
                square.height += square.start_y - y;
                square.start_y = y;
                break;
            case 5: // Right-middle
                square.width = x - square.start_x;
                break;
            case 6: // Bottom-middle
                square.height = y - square.start_y;
                break;
            case 7: // Left-middle
                square.width += square.start_x - x;
                square.start_x = x;
                break;
        }
    }

    // Temporary implementations until Rust functions are available
    Renderer.prototype.get_square_at = function(x, y) { return null; };
    Renderer.prototype.get_squares_in_area = function(x, y, width, height) { return []; };
    Renderer.prototype.update_square = function(square) {};
    Renderer.prototype.remove_square = function(square) {};

    // Initial setup
    resizeCanvas();
    updateCursor();
}

run();