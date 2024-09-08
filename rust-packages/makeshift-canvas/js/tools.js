document.getElementById('pan-mode').addEventListener('click', () => {
    canvas.isDrawingMode = false;
    isPanning = true;
    canvas.hoverCursor = 'move';
});

document.getElementById('draw-mode').addEventListener('click', () => {
    canvas.isDrawingMode = true;
    isPanning = false;
    canvas.freeDrawingBrush.width = 5;
    canvas.hoverCursor = 'crosshair';
});

document.getElementById('eraser-mode').addEventListener('click', () => {
    canvas.isDrawingMode = true;
    isPanning = false;
    canvas.freeDrawingBrush = new fabric.EraserBrush(canvas);
    canvas.freeDrawingBrush.width = 10;
    canvas.hoverCursor = 'crosshair';
});

document.getElementById('rectangle-mode').addEventListener('click', () => {
    canvas.isDrawingMode = false;
    isPanning = false;
    addShape('rect');
});

document.getElementById('circle-mode').addEventListener('click', () => {
    canvas.isDrawingMode = false;
    isPanning = false;
    addShape('circle');
});

document.getElementById('text-mode').addEventListener('click', () => {
    canvas.isDrawingMode = false;
    isPanning = false;
    addTextBox();
});

document.getElementById('color-picker').addEventListener('change', (e) => {
    canvas.freeDrawingBrush.color = e.target.value;
});