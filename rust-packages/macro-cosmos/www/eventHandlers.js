
// this is serving as the state variable for now
const Mode = Object.freeze({
    SELECT: 'select',
    SQUARE: 'square'
});
let currentMode = Mode.SELECT;



export function setupEventHandlers(renderer) {
    const squareBtn = document.getElementById('squareBtn');
    const selectBtn = document.getElementById('selectBtn');
    // const dragBtn = document.getElementById('dragBtn');
    // const editBtn = document.getElementById('editBtn');

    // mode changes
    squareBtn.addEventListener('click', () => {
        currentMode = Mode.SQUARE;
        document.body.style.cursor = 'default'
    });

    // the hand button
    selectBtn.addEventListener('click', () => {
        currentMode = Mode.SELECT;
        // change to the hand cursor called pointer
        document.body.style.cursor = 'pointer'
    });

    renderer.stage.on('mousedown touchstart', (e) => {
        if (currentMode === Mode.SQUARE) {
            renderer.startDrawing();
        } else if (currentMode === Mode.SELECT) {
            renderer.startSelecting();
        }
    });

    renderer.stage.on('mousemove touchmove', (e) => {
        if (currentMode === Mode.SQUARE) {
            renderer.continueDrawing();
        } else if (currentMode == Mode.SELECT) {
            renderer.continueSelecting();
        }
    });

    renderer.stage.on('mouseup touchend', (e) => {
        if (currentMode === Mode.SQUARE) {
            renderer.endDrawing();
        } else if (currentMode == Mode.SELECT) {
            renderer.endSelecting();
        }
    });
}
