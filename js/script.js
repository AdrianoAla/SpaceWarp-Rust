let grid = [];
let eraser = false;
let selected = "⬆️"
let tileImages = {};

function preload() {
    tileImages['⬆️'] = loadImage('img/metal/editor/square/top.png');
    tileImages['⬇️'] = loadImage('img/metal/editor/square/bottom.png');
    tileImages['⬅️'] = loadImage('img/metal/editor/square/left.png');
    tileImages['➡️'] = loadImage('img/metal/editor/square/right.png');
    tileImages['↖️'] = loadImage('img/metal/editor/square/top-left.png');
    tileImages['↗️'] = loadImage('img/metal/editor/square/top-right.png');
    tileImages['↙️'] = loadImage('img/metal/editor/square/bottom-left.png');
    tileImages['↘️'] = loadImage('img/metal/editor/square/bottom-right.png');
    tileImages['⏹️'] = loadImage('img/metal/editor/square/center.png');
    tileImages['⏪'] = loadImage('img/metal/editor/bottom/left.png');
    tileImages['0️⃣'] = loadImage('img/metal/editor/bottom/center.png');
    tileImages['⏩'] = loadImage('img/metal/editor/bottom/right.png');
    tileImages['⏫'] = loadImage('img/metal/editor/top/top.png');
    tileImages['1️⃣'] = loadImage('img/metal/editor/top/center.png');
    tileImages['⏬'] = loadImage('img/metal/editor/top/bottom.png');
    tileImages['⏺️'] = loadImage('img/metal/editor/single.png');
    tileImages['2️⃣'] = loadImage('img/metal/editor/corner/top-left.png');
    tileImages['3️⃣'] = loadImage('img/metal/editor/corner/top-right.png');
    tileImages['4️⃣'] = loadImage('img/metal/editor/corner/bottom-left.png');
    tileImages['5️⃣'] = loadImage('img/metal/editor/corner/bottom-right.png');

    tileImages['👆'] = loadImage('img/metal/editor/fire/up.png');
    tileImages['👇'] = loadImage('img/metal/editor/fire/down.png');
    tileImages['👈'] = loadImage('img/metal/editor/fire/left.png');
    tileImages['👉'] = loadImage('img/metal/editor/fire/right.png');

    tileImages['🟨'] = loadImage('img/editor/yellow/door.png');
    tileImages['🟡'] = loadImage('img/editor/yellow/button.png');
    tileImages['💛'] = loadImage('img/editor/yellow/key.png');

    tileImages['🟥'] = loadImage('img/editor/red/door.png');
    tileImages['🔴'] = loadImage('img/editor/red/button.png');
    tileImages['❤️'] = loadImage('img/editor/red/key.png');

    tileImages['🟦'] = loadImage('img/editor/blue/door.png');
    tileImages['🔵'] = loadImage('img/editor/blue/button.png');
    tileImages['💙'] = loadImage('img/editor/blue/key.png');

}

function setup() {
    const canvas = createCanvas(400, 400);
    canvas.parent('editor');

    for (let i = 0; i < 16; i++) {
        grid.push([]);
        for (let j = 0; j < 16; j++) {
            grid[i].push('⬜');
        }
    }
}

function draw() {
    background(220);

    for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 16; j++) {
            noStroke();
            noSmooth();

            let tile = grid[j][i];
            let x = i * 25;
            let y = j * 25;

            if (tileImages[tile]) {
                image(tileImages[tile], x, y, 25, 25);
            } else {
                fill(255);
                rect(x, y, 25, 25);
            }
        }
    }

    let gx = int((mouseX - mouseX % 25) / 25);
    let gy = int((mouseY - mouseY % 25) / 25);

    if (mouseIsPressed) {
        if (gx < 0 || gx > 15 || gy < 0 || gy > 15) return;
        else if (eraser) grid[gy][gx] = '⬜';
        else {
            if ((selected === '🟨' || selected === '🟥' || selected === '🟦') && (gy > 0 && grid[gy - 1][gx] !== '⬜')) return;
            else if ((gy < 15 && grid[gy + 1][gx] === '🟨') || (gy < 15 && grid[gy + 1][gx] === '🟥') || (gy < 15 && grid[gy + 1][gx] === '🟦')) return;

            if (selected === '⬆️') grid[gy][gx] = '⬆️';
            else if (selected === '⬇️') grid[gy][gx] = '⬇️';
            else if (selected === '⬅️') grid[gy][gx] = '⬅️';
            else if (selected === '➡️') grid[gy][gx] = '➡️';
            else if (selected === '↖️') grid[gy][gx] = '↖️';
            else if (selected === '↗️') grid[gy][gx] = '↗️';
            else if (selected === '↙️') grid[gy][gx] = '↙️';
            else if (selected === '↘️') grid[gy][gx] = '↘️';
            else if (selected === '⏹️') grid[gy][gx] = '⏹️';
            else if (selected === '⏪') grid[gy][gx] = '⏪';
            else if (selected === '0️⃣') grid[gy][gx] = '0️⃣';
            else if (selected === '⏩') grid[gy][gx] = '⏩';
            else if (selected === '⏫') grid[gy][gx] = '⏫';
            else if (selected === '1️⃣') grid[gy][gx] = '1️⃣';
            else if (selected === '⏬') grid[gy][gx] = '⏬';
            else if (selected === '⏺️') grid[gy][gx] = '⏺️';
            else if (selected === '2️⃣') grid[gy][gx] = '2️⃣';
            else if (selected === '3️⃣') grid[gy][gx] = '3️⃣';
            else if (selected === '4️⃣') grid[gy][gx] = '4️⃣';
            else if (selected === '5️⃣') grid[gy][gx] = '5️⃣';
            else if (selected === '👆') grid[gy][gx] = '👆';
            else if (selected === '👇') grid[gy][gx] = '👇';
            else if (selected === '👈') grid[gy][gx] = '👈';
            else if (selected === '👉') grid[gy][gx] = '👉';
            else if (selected === '🟨') grid[gy][gx] = '🟨';
            else if (selected === '🟡') grid[gy][gx] = '🟡';
            else if (selected === '💛') grid[gy][gx] = '💛';
            else if (selected === '🟥') grid[gy][gx] = '🟥';
            else if (selected === '🔴') grid[gy][gx] = '🔴';
            else if (selected === '❤️') grid[gy][gx] = '❤️';
            else if (selected === '🟦') grid[gy][gx] = '🟦';
            else if (selected === '🔵') grid[gy][gx] = '🔵';
            else if (selected === '💙') grid[gy][gx] = '💙';
        }
    }

    if (eraser) fill(255, 255, 255, 100);
    else fill(0, 0, 0, 100);

    rect(gx * 25, gy * 25, 25, 25);
}

function keyPressed() {
    if (key == 'e') {
        toggleEraser();
        alert('Eraser ' + eraser);
    }
}

function setObject(value) {
    selected = value;
}

function toggleEraser() {
    eraser = !eraser;
}

function importRoom() {
    navigator.clipboard
    .readText()
    .then((clipboardData) => {
        const clipboardText = Array.from(clipboardData.toString())
            .filter(c => c !== '\u{fe0f}' && c !== '\u{20e3}')
            .join('');

        const rows = clipboardText.split('\n');

        for (let i = 0; i < 16; i++) {
            const row = rows[i];

            for (let j = 0; j < 16; j++) {
                const char = [...row][j];
                grid[i][j] = char
                    .replace('0', '0️⃣')
                    .replace('1', '1️⃣')
                    .replace('2', '2️⃣')
                    .replace('3', '3️⃣')
                    .replace('4', '4️⃣')
                    .replace('5', '5️⃣')
                    .replace('⬆', '⬆️')
                    .replace('⬇', '⬇️')
                    .replace('⬅', '⬅️')
                    .replace('➡', '➡️')
                    .replace('↖', '↖️')
                    .replace('↗', '↗️')
                    .replace('↙', '↙️')
                    .replace('↘', '↘️')
                    .replace('❤', '❤️');
            }
        }

        console.log('Data imported from clipboard successfully.');
    })
    .catch((error) => {
        console.log('Failed to read clipboard data:', error);
    });
}

function exportRoom() {
    let output = '';

    for (let i = 0; i < 16; i++) {
        let s = grid[i].join('');
        output += s + '\n';
    }

    const spawnX = document.getElementById('spawn-x').value;
    const spawnY = document.getElementById('spawn-y').value;
    if (spawnX && spawnY) output += '-1\n-1\n-1\n' + spawnX + '\n' + spawnY;

    print(output);
    navigator.clipboard.writeText(output);
}
