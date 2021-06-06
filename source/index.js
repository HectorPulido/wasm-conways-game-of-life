import init, { Universe, Cell } from './pkg/conways_test.js';
import wasm from './pkg/conways_test.js';

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const WIDTH = 64;
const HEIGHT = 64;

let ctx;
let canvas;
let universe;

async function run() {
    await init();

    canvas = document.getElementById("game-of-life-canvas");
    canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;
    canvas.width = (CELL_SIZE + 1) * WIDTH + 1;
    ctx = canvas.getContext('2d');

    universe = Universe.new(WIDTH, HEIGHT);
    tic();
}

function tic() {
    // pre.textContent = universe.render()
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    universe.tick();
    drawGrid();
    drawCells();
    requestAnimationFrame(tic);
}

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= WIDTH; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= HEIGHT; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

async function drawCells() {
    const cells = universe.cells();

    ctx.beginPath();

    for (let row = 0; row < HEIGHT; row++) {
        for (let col = 0; col < WIDTH; col++) {
            const idx = universe.get_index(row, col);
            console.log(idx);

            ctx.fillStyle = cells[idx] === 0
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

run();