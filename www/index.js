import init, { World } from "snake-game";

init().then(_ => {
    const CELL_SIZE = 20;

    const world = World.new();
    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = world.width() * CELL_SIZE;
    canvas.width = world.width() * CELL_SIZE;

    function drawWorld() {
        ctx.beginPath();

        for (let x = 0; x <= world.width(); ++x) {
            ctx.moveTo(x * CELL_SIZE, 0);
            ctx.lineTo(x * CELL_SIZE, world.width() * CELL_SIZE);
        }

        for (let y = 0; y <= world.width(); ++y) {
            ctx.moveTo(0, y * CELL_SIZE);
            ctx.lineTo(world.width() * CELL_SIZE, y * CELL_SIZE);
        }

        ctx.stroke();
    }

    function drawSnake() {
        const snakeIdx = world.snake_head_idx();
        const col = snakeIdx % world.width();
        const row = Math.floor(snakeIdx / world.width());

        ctx.beginPath();
        ctx.fillRect(
            col * CELL_SIZE, 
            row * CELL_SIZE, 
            CELL_SIZE, 
            CELL_SIZE
        );
        ctx.stroke();
    }

    drawWorld();
    drawSnake();
})