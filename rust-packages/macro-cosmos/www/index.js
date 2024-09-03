import { Renderer } from './renderer.js';
import { setupEventHandlers } from './eventHandlers.js';

const renderer = new Renderer('canvas-container');
setupEventHandlers(renderer);