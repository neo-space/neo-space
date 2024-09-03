import Konva from 'konva';

let shapes = [];
let selectedShapes = [];

export function createShape(x, y, width, height, renderer) {
    return renderer.createShape('rectangle', {
        x, y, width, height,
        fill: 'transparent',
        stroke: 'black',
        strokeWidth: 1,
        draggable: true,
    });
}

export function selectShape(shape, renderer) {
    renderer.selectShape(shape);
}

export function deselectShape(shape, renderer) {
    renderer.deselectAll();
}

function addResizeHandles(shape) {
    const handles = [];
    const positions = [
        { x: 0, y: 0 },
        { x: 1, y: 0 },
        { x: 0, y: 1 },
        { x: 1, y: 1 },
        { x: 0.5, y: 0 },
        { x: 1, y: 0.5 },
        { x: 0.5, y: 1 },
        { x: 0, y: 0.5 },
    ];

    positions.forEach((pos, i) => {
        const handle = new Konva.Rect({
            x: shape.x() + pos.x * shape.width(),
            y: shape.y() + pos.y * shape.height(),
            width: 8,
            height: 8,
            fill: 'white',
            stroke: 'lightblue',
            strokeWidth: 1,
            draggable: true,
            name: 'handle',
        });

        handle.on('dragmove', function () {
            const newPos = this.position();
            let newWidth = shape.width();
            let newHeight = shape.height();
            let newX = shape.x();
            let newY = shape.y();

            if (i < 4) {  // Corner handles
                newWidth = Math.abs(newPos.x - shape.x());
                newHeight = Math.abs(newPos.y - shape.y());
                if (i === 0 || i === 2) newX = newPos.x;
                if (i === 0 || i === 1) newY = newPos.y;
            } else if (i === 4 || i === 6) {  // Top and bottom middle handles
                newHeight = Math.abs(newPos.y - shape.y());
                if (i === 4) newY = newPos.y;
            } else {  // Left and right middle handles
                newWidth = Math.abs(newPos.x - shape.x());
                if (i === 7) newX = newPos.x;
            }

            shape.width(newWidth);
            shape.height(newHeight);
            shape.position({ x: newX, y: newY });
            updateResizeHandles(shape);
            updateRustShape(shape);
            shape.getLayer().batchDraw();
        });

        handles.push(handle);
        shape.getLayer().add(handle);
    });

    shape.setAttr('handles', handles);
}

function removeResizeHandles(shape) {
    const handles = shape.getAttr('handles') || [];
    handles.forEach(handle => handle.destroy());
    shape.setAttr('handles', null);
}

export function updateResizeHandles(shape) {
    const handles = shape.getAttr('handles') || [];
    const positions = [
        { x: 0, y: 0 },
        { x: 1, y: 0 },
        { x: 0, y: 1 },
        { x: 1, y: 1 },
        { x: 0.5, y: 0 },
        { x: 1, y: 0.5 },
        { x: 0.5, y: 1 },
        { x: 0, y: 0.5 },
    ];

    handles.forEach((handle, i) => {
        handle.position({
            x: shape.x() + positions[i].x * shape.width(),
            y: shape.y() + positions[i].y * shape.height(),
        });
    });
}

export function createRandomShapes(count, renderer) {
    for (let i = 0; i < count; i++) {
        const x = Math.random() * renderer.stage.width();
        const y = Math.random() * renderer.stage.height();
        const width = 50 + Math.random() * 50;
        const height = 50 + Math.random() * 50;
        createShape(x, y, width, height, renderer);
    }
    renderer.layer.draw();
}

export function getSelectedShapes(renderer) {
    return renderer.selectedShapes;
}

export function clearSelectedShapes(renderer) {
    renderer.selectedShapes.forEach(shape => renderer.removeShape(shape));
    renderer.selectedShapes = [];
}

export function getShapesInArea(x1, y1, x2, y2, renderer) {
    return renderer.getShapesInArea(x1, y1, x2, y2);
}