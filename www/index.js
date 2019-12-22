import { Model, Cell } from "picawngh";
import { memory } from "picawngh/picawngh_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const model = Model.random(300,300);
const width = model.w();
const height = model.h();

const canvas = document.getElementById("world");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawInit = () => {
    const worldPtr = model.world();
    const cells = new Uint8Arrayt(memory.buffer, worldPtr, width*height);
    ctx.beginPath()
    for (let i = 0; i<height*width; i++) {
        ctx.fillStyle = cells[i] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;
        let col = i % width;
        let row = Math.floor(i/width);
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
    }
}

drawInit();

// const renderLoop = () => {
//   const model.
//
//   drawCells();
//
//   requestAnimationFrame(renderLoop);
// };
//
