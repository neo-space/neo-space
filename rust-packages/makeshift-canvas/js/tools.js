function initTools(canvas) {
    let isPanning = false;
    let isErasing = false;

    document.getElementById('pan-mode').addEventListener('click', () => {
        canvas.isDrawingMode = false;
        isPanning = true;
        isErasing = false;
        canvas.hoverCursor = 'move';
    });

    document.getElementById('draw-mode').addEventListener('click', () => {
        canvas.isDrawingMode = true;
        isPanning = false;
        isErasing = false;
        canvas.freeDrawingBrush = new fabric.PencilBrush(canvas);
        canvas.freeDrawingBrush.color = document.getElementById('color-picker').value;
        canvas.freeDrawingBrush.width = 5;
        canvas.hoverCursor = 'crosshair';
    });

    document.getElementById('eraser-mode').addEventListener('click', () => {
        canvas.isDrawingMode = true;
        isPanning = false;
        isErasing = true;
        canvas.freeDrawingBrush = new fabric.EraserBrush(canvas);
        canvas.freeDrawingBrush.width = 20;
        canvas.hoverCursor = 'crosshair';
    });

    document.getElementById('rectangle-mode').addEventListener('click', () => {
        canvas.isDrawingMode = false;
        isPanning = false;
        isErasing = false;
        addShape('rect', canvas);
    });

    document.getElementById('circle-mode').addEventListener('click', () => {
        canvas.isDrawingMode = false;
        isPanning = false;
        isErasing = false;
        addShape('circle', canvas);
    });

    document.getElementById('text-mode').addEventListener('click', () => {
        canvas.isDrawingMode = false;
        isPanning = false;
        isErasing = false;
        addTextBox(canvas);
    });

    document.getElementById('add-gif').addEventListener('click', () => {
        const gifUrl = prompt("Enter the URL of the GIF:");
        if (gifUrl) {
            addGifToCanvas(canvas, gifUrl);
        }
    });

    document.getElementById('color-picker').addEventListener('change', (e) => {
        if (!isErasing) {
            canvas.freeDrawingBrush.color = e.target.value;
        }
    });

    canvas.on('path:created', function(e) {
        e.path.set({
            erasable: true
        });
    });

    // Make all existing objects on the canvas erasable
    canvas.getObjects().forEach(obj => {
        obj.set('erasable', true);
    });

    // Set up infinite panning
    canvas.on('mouse:down', function(opt) {
        const evt = opt.e;
        if (isPanning) {
            this.isDragging = true;
            this.selection = false;
            this.lastPosX = evt.clientX;
            this.lastPosY = evt.clientY;
        }
    });

    canvas.on('mouse:move', function(opt) {
        if (this.isDragging) {
            const e = opt.e;
            const vpt = this.viewportTransform;
            vpt[4] += e.clientX - this.lastPosX;
            vpt[5] += e.clientY - this.lastPosY;
            this.requestRenderAll();
            this.lastPosX = e.clientX;
            this.lastPosY = e.clientY;
        }
    });

    canvas.on('mouse:up', function(opt) {
        this.isDragging = false;
        this.selection = true;
    });

    // Zoom functionality
    canvas.on('mouse:wheel', function(opt) {
        var delta = opt.e.deltaY;
        var zoom = canvas.getZoom();
        zoom *= 0.999 ** delta;
        if (zoom > 20) zoom = 20;
        if (zoom < 0.01) zoom = 0.01;
        canvas.zoomToPoint({ x: opt.e.offsetX, y: opt.e.offsetY }, zoom);
        opt.e.preventDefault();
        opt.e.stopPropagation();
    });
}

function addShape(shapeType, canvas) {
    let shape;
    if (shapeType === 'rect') {
        shape = new fabric.Rect({
            left: 100,
            top: 100,
            fill: document.getElementById('color-picker').value,
            width: 100,
            height: 100,
            erasable: true
        });
    } else if (shapeType === 'circle') {
        shape = new fabric.Circle({
            left: 100,
            top: 100,
            fill: document.getElementById('color-picker').value,
            radius: 50,
            erasable: true
        });
    }
    canvas.add(shape);
    canvas.renderAll();
}

function addTextBox(canvas) {
    const text = new fabric.Textbox('Click to edit (Supports Markdown)', {
        left: 100,
        top: 100,
        width: 200,
        fontSize: 16,
        erasable: true
    });

    text.on('editing:exited', function() {
        const markdown = this.text;
        const html = marked.parse(markdown);
        this.set('text', html);
        canvas.renderAll();
    });

    canvas.add(text);
    canvas.renderAll();
}

function addGifToCanvas(canvas, url) {
    fabric.Image.fromURL(url, function(img) {
        img.set({
            left: 100,
            top: 100,
            erasable: true
        });

        // Create a wrapper element to hold the GIF
        const wrapper = document.createElement('div');
        wrapper.style.position = 'absolute';
        wrapper.style.pointerEvents = 'none'; // Allow interaction with canvas beneath

        // Create an img element for the GIF
        const gifImg = document.createElement('img');
        gifImg.src = url;
        gifImg.style.width = '100%';
        gifImg.style.height = '100%';
        wrapper.appendChild(gifImg);

        // Add the wrapper to the document body
        document.body.appendChild(wrapper);

        // Create a custom fabric object
        const gifObject = new fabric.Image(img.getElement(), {
            left: 100,
            top: 100,
            width: img.width,
            height: img.height,
            erasable: true
        });

        // Update wrapper position when the object moves
        gifObject.on('moving', function() {
            updateWrapperPosition();
        });
        gifObject.on('scaling', function() {
            updateWrapperPosition();
        });

        function updateWrapperPosition() {
            const zoom = canvas.getZoom();
            const pan = canvas.viewportTransform;
            wrapper.style.left = (gifObject.left * zoom + pan[4]) + 'px';
            wrapper.style.top = (gifObject.top * zoom + pan[5]) + 'px';
            wrapper.style.width = (gifObject.width * gifObject.scaleX * zoom) + 'px';
            wrapper.style.height = (gifObject.height * gifObject.scaleY * zoom) + 'px';
        }

        // Initial position update
        updateWrapperPosition();

        // Add the object to canvas
        canvas.add(gifObject);
        canvas.renderAll();

        // Update wrapper position on canvas pan/zoom
        canvas.on('viewportTransform', updateWrapperPosition);
    });
}