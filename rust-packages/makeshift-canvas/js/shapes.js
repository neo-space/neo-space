function addShape(shapeType) {
    let shape;
    if (shapeType === 'rect') {
        shape = new fabric.Rect({
            left: 100,
            top: 100,
            fill: document.getElementById('color-picker').value,
            width: 100,
            height: 100
        });
    } else if (shapeType === 'circle') {
        shape = new fabric.Circle({
            left: 100,
            top: 100,
            fill: document.getElementById('color-picker').value,
            radius: 50
        });
    }
    canvas.add(shape);
    canvas.renderAll();
}