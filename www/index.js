import init, { greet } from "snake-game";

init().then(_ => {
    greet("Roman");
    console.log("Ok!");
})