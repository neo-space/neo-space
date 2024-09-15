import '../styles.css';

document.addEventListener('DOMContentLoaded', () => {
    const gridContainer = document.getElementById('grid-container');
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    let scale = 1;
    let translateX = 0;
    let translateY = 0;

    const MAX_SCALE = 5;
    const MIN_SCALE = 0.25;

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        update();
    }

    function updateGrid() {
        const baseBgSize = 20;
        let bgSize = baseBgSize * scale;
        
        let lodLevel = 0;
        while (bgSize < 10 && lodLevel < 3) {
            bgSize *= 2;
            lodLevel++;
        }
        
        const offsetX = (translateX * scale) % bgSize;
        const offsetY = (translateY * scale) % bgSize;
        
        gridContainer.style.backgroundPosition = `${offsetX}px ${offsetY}px`;
        gridContainer.style.transform = `scale(${scale})`;
        gridContainer.style.backgroundSize = `${bgSize}px ${bgSize}px`;
    }

    function update() {
        updateGrid();
    }

    // Pan functionality
    let isDragging = false;
    let lastX, lastY;

    gridContainer.addEventListener('mousedown', (e) => {
        isDragging = true;
        lastX = e.clientX;
        lastY = e.clientY;
    });

    window.addEventListener('mousemove', (e) => {
        if (isDragging) {
            const deltaX = e.clientX - lastX;
            const deltaY = e.clientY - lastY;
            translateX += deltaX / scale;
            translateY += deltaY / scale;
            lastX = e.clientX;
            lastY = e.clientY;
            update();
        }
    });

    window.addEventListener('mouseup', () => {
        isDragging = false;
    });

    // Zoom functionality
    gridContainer.addEventListener('wheel', (e) => {
        e.preventDefault();
        const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
        
        const rect = gridContainer.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;
        
        let newScale = scale * zoomFactor;
        newScale = Math.max(MIN_SCALE, Math.min(MAX_SCALE, newScale));
        
        if (newScale !== scale) {
            const zoomFactor = newScale / scale;
            translateX += (mouseX / scale) * (1 - 1 / zoomFactor);
            translateY += (mouseY / scale) * (1 - 1 / zoomFactor);
            scale = newScale;
            update();
        }
    }, { passive: false });

    // Initial setup
    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    // Initial update
    update();
});