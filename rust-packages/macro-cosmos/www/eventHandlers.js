let currentMode = 'drag';

export function setupEventHandlers(renderer) {
    const squareBtn = document.getElementById('squareBtn');
    // const dragBtn = document.getElementById('dragBtn');
    // const editBtn = document.getElementById('editBtn');

    // mode changes
    squareBtn.addEventListener('click', () => {
        currentMode = 'square';
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

    setupKeyboardEvents(renderer);
}
