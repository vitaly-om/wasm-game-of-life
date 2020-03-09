import { Universe } from "wasm-game-of-life";

const canvas = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
    canvas.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
