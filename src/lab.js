const invoke = window.__TAURI__.invoke


document.querySelector('#btn-alert').addEventListener('click', ()=>{
    console.log(document.querySelector('#btn-alert'))
    console.log('Haz dado click')
    alert('Haz dado click');
})

document.querySelector('#btn-alert-backend').addEventListener('click', ()=>{
    invoke ("helloworld", {name: (document.querySelector('#name').value || 'vacío')});
})

document.querySelector('#btn-get-from-backend').addEventListener('click', async ()=>{
    const message = await invoke ("mensajeDesdeElBackend", {mensaje: (document.querySelector('#name').value || '')});
    document.querySelector('#lienzo').innerHTML = message;
})
