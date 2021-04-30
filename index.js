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

        const FPS_THROTTLE = 1000.0 / 30.0 // milliseconds / frames
        const langasGame = new m.LangasGame();
        const initialTime = Date.now();

        let lastDrawTime = -1 // in milliseconds

        function render() {
            window.requestAnimationFrame(render);

            const currentTime = Date.now();

            if (currentTime >= lastDrawTime + FPS_THROTTLE) {
                lastDrawTime = currentTime;

                if (window.innerHeight !== canvas_2d.height || window.innerWidth !== canvas_2d.width) {
                    canvas_2d.height = window.innerHeight;
                    canvas_2d.clientHeight = window.innerHeight;
                    canvas_2d.style.height = window.innerHeight;
    
                    canvas_2d.width = window.innerWidth;
                    canvas_2d.clientWidth = window.innerWidth;
                    canvas_2d.style.width = window.innerWidth;
    
                    gl.viewport(0, 0, window.innerWidth, window.innerHeight);
                }

                const elapsedTime = currentTime - initialTime;
                langasGame.update(elapsedTime, window.innerHeight, window.innerWidth)
                langasGame.render();
            }
        }

        render();
    })
    .catch(e => console.error(e));