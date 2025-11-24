import init, { test } from './pkg/hcc07_org_site.js'

async function run() {
  await init("./pkg/hcc07_org_site_bg.wasm");
    const outputDiv = document.getElementById("output");
    outputDiv.innerText = test();
}

run();
