function initCanvas() {
    const canvas = new fabric.Canvas('canvas', {
        width: window.innerWidth,
        height: window.innerHeight,
        isDrawingMode: false
    });

    let isPanning = false;
    let panOffsetX = 0;
    let panOffsetY = 0;

    // Set up dot grid background
    function createDotGrid() {
        const gridSize = 20;
        const dotSize = 1;
        const gridCanvas = document.createElement('canvas');
        gridCanvas.width = gridSize;
        gridCanvas.height = gridSize;
        const ctx = gridCanvas.getContext('2d');
        ctx.fillStyle = 'rgba(0, 0, 0, 0.1)';
        ctx.fillRect(0, 0, dotSize, dotSize);
        return gridCanvas;
    }

    const dotGridPattern = createDotGrid();
    document.body.style.backgroundImage = `url(${dotGridPattern.toDataURL()})`;
    document.body.style.backgroundSize = '20px 20px';
    document.body.style.backgroundPosition = '0 0';

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

            // Update dot grid position
            panOffsetX += e.clientX - this.lastPosX;
            panOffsetY += e.clientY - this.lastPosY;
            document.body.style.backgroundPosition = `${panOffsetX}px ${panOffsetY}px`;
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

        // Update dot grid size
        const gridSize = 20 * zoom;
        document.body.style.backgroundSize = `${gridSize}px ${gridSize}px`;
    });

    // File drag and drop
    canvas.wrapperEl.addEventListener('dragover', (e) => {
        e.preventDefault();
        e.stopPropagation();
    });

    canvas.wrapperEl.addEventListener('drop', (e) => {
        e.preventDefault();
        e.stopPropagation();
        
        const files = e.dataTransfer.files;
        for (let file of files) {
            if (file.type.startsWith('image/')) {
                addImageToCanvas(file, canvas);
            } else {
                addFileIconToCanvas(file, canvas);
            }
        }
    });

    return canvas;
}

function addImageToCanvas(file, canvas) {
    const reader = new FileReader();
    reader.onload = (e) => {
        fabric.Image.fromURL(e.target.result, (img) => {
            img.scale(0.5);  // Scale down the image
            canvas.add(img);
            canvas.renderAll();
        });
    };
    reader.readAsDataURL(file);
}

function addFileIconToCanvas(file, canvas) {
    const icon = new fabric.Text(file.name, {
        left: 100,
        top: 100,
        fill: 'black',
        fontSize: 16
    });
    canvas.add(icon);
    canvas.renderAll();
}