import init, { run } from "./pkg/chip_8_wasm.js";
init().then(() => {
    run();
});
