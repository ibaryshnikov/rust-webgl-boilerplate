import init, { Scene } from './pkg/index.js';

window.addEventListener('load', async () => {
  await init();

  console.log('wasm loaded');

  const scene = Scene.new();
  scene.draw();
});
