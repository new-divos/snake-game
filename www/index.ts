import init, { Direction, World } from "snake-game";
import { rnd } from "./utils/rnd"

init().then(_ => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 16;
    const SNAKE_SIZE = 3;

    const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

    const world = World.new(WORLD_WIDTH, snakeSpawnIdx, SNAKE_SIZE);

    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    const gameControlBtn = document.getElementById("game-control-btn");
    const gameStatusLbl = document.getElementById("game-status");

    canvas.height = world.width() * CELL_SIZE;
    canvas.width = world.width() * CELL_SIZE;

    gameControlBtn.addEventListener("click", _ => {
        const gameStatus = world.game_status();

        if (gameStatus === undefined) {
            gameControlBtn.textContent = "Playing...";

            world.start_game();
            play();
        } else {
            location.reload();
        }
    })

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

    function drawReward() {
        const idx = world.reward_cell();

        if (idx > world.size()) {
            alert("You Won!");
        } else {
            const col = idx % world.width();
            const row = Math.floor(idx / world.width());    

            ctx.beginPath();
            ctx.fillStyle = "#FF0000";
            ctx.fillRect(
                col * CELL_SIZE, 
                row * CELL_SIZE, 
                CELL_SIZE, 
                CELL_SIZE
            );
            ctx.stroke();    
        }
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

    function drawGameStatus() {
        gameStatusLbl.textContent = world.game_status_text();
    }

    function paint() {
        drawWorld();
        drawSnake();
        drawReward();
        drawGameStatus();
    }

    function play() {
        const fps = 3;

        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            paint();

            // the method takes a callback to invoked before the next repaint
            requestAnimationFrame(play);
        }, 1000 / fps);
    }

    paint();
})