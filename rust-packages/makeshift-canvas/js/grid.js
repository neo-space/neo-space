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