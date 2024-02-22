import { Universe } from './pkg';
import { memory } from "./pkg/index_bg.wasm";



const canvas = document.getElementById("world-canvas");
const ctx = canvas.getContext('2d');
const universe = Universe.new();
canvas.width = universe.width();
canvas.height = universe.height();

canvas.addEventListener("click", e => {
  universe.add_particle(e.clientX, e.clientY)
  draw()
})

// const renderLoop = () => {
//   draw();
//   requestAnimationFrame(renderLoop);
// }

const draw = () => {
  const particlesPtr = universe.particles();
  const count = universe.size()
  const cells = new Float32Array(memory.buffer, particlesPtr, count * 2);

  for (let idx = 0; idx < count; idx++ ) {
    const x = cells[idx * 2]
    const y = cells[idx * 2 + 1]
    ctx.beginPath();
    ctx.arc(x, y, 20, 0, Math.PI * 2, true);
    ctx.stroke();
    ctx.fill();
  }
}