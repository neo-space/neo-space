
import init, { Renderer } from '../pkg/macro_cosmos.js';

async function run() {
    await init();
    const renderer = new Renderer();
    
    let isDrawing = false;
    let startX, startY;

    const canvas = document.getElementById('canvas');
    const rect = canvas.getBoundingClientRect();

    function getNormalizedCoords(clientX, clientY) {
        const x = (clientX - rect.left) / rect.width * 2 - 1;
        const y = -((clientY - rect.top) / rect.height * 2 - 1);
        return [x, y];
    }

    canvas.addEventListener('mousedown', (e) => {
        isDrawing = true;
        [startX, startY] = getNormalizedCoords(e.clientX, e.clientY);
    });

    canvas.addEventListener('mouseup', (e) => {
        if (isDrawing) {
            const [endX, endY] = getNormalizedCoords(e.clientX, e.clientY);
            renderer.draw_square(startX, startY, endX, endY);
            isDrawing = false;
        }
    });
}

run();