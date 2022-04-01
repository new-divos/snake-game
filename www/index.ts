import init, { Direction, World } from "snake-game";

init().then(_ => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 16;
    const SNAKE_SIZE = 3;

    const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

    const world = World.new(WORLD_WIDTH, snakeSpawnIdx, SNAKE_SIZE);
    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = world.width() * CELL_SIZE;
    canvas.width = world.width() * CELL_SIZE;

    document.addEventListener("keydown", e => {
        switch (e.code) {
            case "ArrowUp":
                world.change_shake_dir(Direction.Up);
                break;

            case "ArrowRight":
                world.change_shake_dir(Direction.Right);
                break;

            case "ArrowDown":
                world.change_shake_dir(Direction.Down);
                break;

            case "ArrowLeft":
                world.change_shake_dir(Direction.Left);
                break;
        }
    });

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
        const snakeCells = world.snake_cells();
        snakeCells.forEach(cellIdx => {
            const col = cellIdx % world.width();
            const row = Math.floor(cellIdx / world.width());

            ctx.fillStyle = (cellIdx === world.snake_head_idx() 
                ? "#7878db" : "#009966");
    
            ctx.beginPath();
            ctx.fillRect(
                col * CELL_SIZE, 
                row * CELL_SIZE, 
                CELL_SIZE, 
                CELL_SIZE
            );
            ctx.stroke();    
        });
    }

    function paint() {
        drawWorld();
        drawSnake();
    }

    function update() {
        const fps = 3;

        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            paint();

            // the method takes a callback to invoked before the next repaint
            requestAnimationFrame(update);
        }, 1000 / fps);
    }

    paint();
    update();
})