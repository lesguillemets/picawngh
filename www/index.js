import { Model, Cell } from "picawngh";
import { memory } from "picawngh/picawngh_bg";

const CELL_SIZE = 5;
const DEAD_COLOUR = "#0f2e1c";
const ALIVE_COLOUR = "#169e51";

const model = Model.random(300,300);
const width = model.w();
const height = model.h();

const canvas = document.getElementById("world");
canvas.height = CELL_SIZE * height;
canvas.width = CELL_SIZE * width;

const ctx = canvas.getContext('2d');

const drawInit = () => {
    const worldPtr = model.world();
    const cells = new Uint8Array(memory.buffer, worldPtr, width*height);
    for (let i = 0; i<height*width; i++) {
        ctx.fillStyle = cells[i] === Cell.Dead
          ? DEAD_COLOUR
          : ALIVE_COLOUR;
        let col = i % width;
        let row = Math.floor(i/width);
        ctx.fillRect(
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );
    }
}

drawInit();

const renderLoop = () => {
    const diffPtr = model.update_and_report();
    const update_count = model.tell_last_update_count();
    const diff = new Int32Array(memory.buffer, diffPtr, update_count);
    for (let i = 0; i<update_count; i++) {
        ctx.fillStyle = diff[i] < 0
          ? DEAD_COLOUR
          : ALIVE_COLOUR;
        let loc = Math.abs(diff[i]) -1;
        let col =  loc % width;
        let row = Math.floor(loc/width);
        ctx.fillRect(
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );
    }
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
