import('../pkg')
    .then(wasm => {
        console.log('wasm loaded');
        const result = wasm.add(2, 3);
        console.log(`result`, result);
    })
    .catch(e => console.log('error loading wasm', e));
