import init, { Scene } from './pkg/webgl_boilerplate.js';

window.addEventListener('load', async () => {
  await init();

  console.log('wasm loaded');

  const scene = Scene.new();
  scene.draw();
});
