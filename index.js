const rust_layer = import('./pkg/rust_layer');
const canvas_2d = document.getElementById('canvas-2d')
const gl = canvas_2d.getContext('webgl', { antialias: true })

rust_layer
    .then(m => {

        m.greet("Hola")

        if (!gl) {
            alert("Failed to initialize WebGl");
            return;
        }

        gl.enable(gl.BLEND);
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUES_SRC_ALPHA);

        const FPS_THROTTLE = 1000.0 / 30.0 // milliseconds / frames
        const langasGame = new m.LangasGame();
        console.log("faz algo", langasGame);
        const initialTime = Date.now();

        let lastDrawTime = -1 // in milliseconds

        function render() {
            window.requestAnimationFrame(render);

            const currentTime = Date.now();

            if (currentTime >= lastDrawTime + FPS_THROTTLE) {
                lastDrawTime = currentTime;

                // TODO resize canvas with window
                // if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                gl.viewport(0, 0, window.innerWidth, window.innerHeight)
                // }
                const elapsedTime = currentTime - initialTime;
                langasGame.update(elapsedTime, window.innerHeight, window.innerWidth)
                langasGame.render();
            }
        }

        render();
    })
    .catch(e => console.error(e));