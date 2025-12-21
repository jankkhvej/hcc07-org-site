import init, { test } from './pkg/client.js'

async function run() {
  await init("./pkg/client_bg.wasm");
    const outputDiv = document.getElementById("output");
    outputDiv.innerText = test();
}

run();
