import('../pkg')
    .then(wasm => {
        console.log('wasm loaded');

        const scene = wasm.Scene.new();
        scene.draw();
    })
    .catch(e => console.log('error loading wasm', e));
