let currentMode = 'drag';

export function setupEventHandlers(renderer) {
    const squareBtn = document.getElementById('squareBtn');
    const dragBtn = document.getElementById('dragBtn');
    const editBtn = document.getElementById('editBtn');

    squareBtn.addEventListener('click', () => {
        currentMode = 'square';
        renderer.setDraggable(false);
    });

    dragBtn.addEventListener('click', () => {
        currentMode = 'drag';
        renderer.setDraggable(true);
    });

    editBtn.addEventListener('click', () => {
        currentMode = 'edit';
        renderer.setDraggable(false);
    });

    renderer.stage.on('mousedown touchstart', (e) => {
        if (currentMode === 'square') {
            const pos = renderer.stage.getPointerPosition();
            renderer.startDrawing(pos);
        } else if (currentMode === 'edit') {
            if (e.target === renderer.stage) {
                renderer.deselectAll();
            }
        }
    });

    renderer.stage.on('mousemove touchmove', () => {
        if (currentMode === 'square') {
            const pos = renderer.stage.getPointerPosition();
            renderer.continueDrawing(pos);
        }
    });

    renderer.stage.on('mouseup touchend', () => {
        if (currentMode === 'square') {
            renderer.endDrawing();
        }
    });

    renderer.stage.on('click tap', (e) => {
        if (currentMode === 'edit' && e.target !== renderer.stage) {
            renderer.selectShape(e.target);
        }
    });

    setupKeyboardEvents(renderer);
}

function setupKeyboardEvents(renderer) {
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Delete' && currentMode === 'edit') {
            renderer.selectedShapes.forEach(shape => {
                renderer.removeShape(shape);
            });
            renderer.selectedShapes = [];
            renderer.layer.draw();
        }
    });
}