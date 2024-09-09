document.addEventListener('DOMContentLoaded', function() {
    const canvas = initCanvas();
    initTools(canvas);

    // Resize canvas on window resize
    window.addEventListener('resize', () => {
        canvas.setDimensions({
            width: window.innerWidth,
            height: window.innerHeight
        });
    });
});