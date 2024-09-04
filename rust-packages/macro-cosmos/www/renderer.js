import Konva from 'konva';

export class Renderer {
    constructor(containerId) {
        this.stage = new Konva.Stage({
            container: containerId,
            width: window.innerWidth,
            height: window.innerHeight,
        });

        this.layer = new Konva.Layer();
        this.stage.add(this.layer);

        this.shapes = [];
        this.selectedShapes = [];
        this.isDrawing = false;
        this.drawingShape = null;

        this.setupResizeHandler();
    }

    setupResizeHandler() {
        window.addEventListener('resize', () => {
            this.stage.width(window.innerWidth);
            this.stage.height(window.innerHeight);
            this.layer.draw();
        });
    }

    createShape(type, config) {
        let shape;
        switch (type) {
            case 'rectangle':
                shape = new Konva.Rect({
                    ...config,
                    fill: 'pink',
                    stroke: 'pink',
                    strokeWidth: 2,        
                    shadowBlur: 10,
                    cornerRadius: 10,
                });
                break;
            default:
                console.error('Unsupported shape type');
                return null;
        }

        this.shapes.push(shape);
        this.layer.add(shape);
        this.layer.draw();
        return shape;
    }

    startDrawing(pos) {
        this.isDrawing = true;
        this.drawingShape = this.createShape('rectangle', {
            x: pos.x,
            y: pos.y,
            width: 0,
            height: 0,
        });
    }

    continueDrawing(pos) {
        if (!this.isDrawing) return;
        const rect = this.drawingShape;
        const newWidth = pos.x - rect.x();
        const newHeight = pos.y - rect.y();
        rect.width(newWidth);
        rect.height(newHeight);
        this.layer.batchDraw();
    }

    endDrawing() {
        this.isDrawing = false;
        this.drawingShape = null;
    }

    selectShape(shape) {
        this.deselectAll();
        shape.stroke('blue');
        shape.strokeWidth(3);
        this.selectedShapes.push(shape);
        this.addTransformer(shape);
        this.layer.draw();
    }

    deselectAll() {
        this.selectedShapes.forEach(shape => {
            shape.stroke('black');
            shape.strokeWidth(2);
        });
        this.selectedShapes = [];
        this.removeTransformers();
        this.layer.draw();
    }

    addTransformer(shape) {
        const tr = new Konva.Transformer({
            nodes: [shape],
            keepRatio: false,
            boundBoxFunc: (oldBox, newBox) => {
                if (newBox.width < 5 || newBox.height < 5) {
                    return oldBox;
                }
                return newBox;
            },
        });
        this.layer.add(tr);
    }

    removeTransformers() {
        this.stage.find('Transformer').destroy();
    }

    getShapesInArea(x1, y1, x2, y2) {
        const box = new Konva.Rect({
            x: Math.min(x1, x2),
            y: Math.min(y1, y2),
            width: Math.abs(x2 - x1),
            height: Math.abs(y2 - y1),
        });

        return this.shapes.filter(shape => 
            Konva.Util.haveIntersection(box.getClientRect(), shape.getClientRect())
        );
    }

    removeShape(shape) {
        this.shapes = this.shapes.filter(s => s !== shape);
        shape.destroy();
        this.layer.draw();
    }

    clear() {
        this.shapes.forEach(shape => shape.destroy());
        this.shapes = [];
        this.selectedShapes = [];
        this.layer.draw();
    }

    toJSON() {
        return this.stage.toJSON();
    }

    loadFromJSON(json) {
        const stage = Konva.Node.create(json, 'container');
        this.stage.destroy();
        this.stage = stage;
        this.layer = this.stage.findOne('Layer');
        this.shapes = this.layer.getChildren();
        this.selectedShapes = [];
    }
}