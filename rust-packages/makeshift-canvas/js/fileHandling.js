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
            addImageToCanvas(file);
        } else {
            addFileIconToCanvas(file);
        }
    }
});

function addImageToCanvas(file) {
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

function addFileIconToCanvas(file) {
    const icon = new fabric.Text(file.name, {
        left: 100,
        top: 100,
        fill: 'black',
        fontSize: 16
    });
    canvas.add(icon);
    canvas.renderAll();
}