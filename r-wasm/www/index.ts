import init, {Universe, Cell} from '../pkg/r_wasm.js'

const CELL_SIZE = 10;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
init().then((_exports) => {

    const universe = Universe.new();
    const width =  universe.width();
    const height = universe.height();
    const canvas = document.getElementById('game-of-life-canvas') as HTMLCanvasElement;
    canvas.height = (CELL_SIZE + 1) * height +1;
    canvas.width = (CELL_SIZE +1) * width +1;

    const ctx = canvas.getContext('2d');

    const drawGrid = ()=>{
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        for (let i = 0; i <= width; i++) {
            ctx.moveTo(i* (CELL_SIZE +1)+1, 0);
            ctx.lineTo(i*(CELL_SIZE +1)+1, (CELL_SIZE +1)* height +1)
        }

        // Horizontal lines.
        for (let j = 0; j <= height; j++) {
            ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
            ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
        }

        ctx.stroke();

    }


    const getIndex = (row, column) => {
        return row * width + column;
    };

    const drawCells = () => {
        const cellsPtr = universe.cells();
        // https://juejin.cn/post/7080898368712491038
        debugger
        const cells = new Uint8Array(_exports.memory.buffer, cellsPtr, width * height);
        debugger
        ctx.beginPath();

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);

                ctx.fillStyle = cells[idx] === Cell.Dead
                    ? DEAD_COLOR
                    : ALIVE_COLOR;

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        ctx.stroke();
    };

    const loop = ()=>{

        universe.tick();
        drawGrid();
        drawCells();

        requestAnimationFrame(loop)
    }

    loop()


})

