let grid = [];
let eraser = false;
let selected = "⬆️"
let tileImages = {};
let texturePack = "metal";


function preload() {
    loadTileImages();
}

function loadTileImages() {
    const texturePath = texturePack === "natural" ? "img/natural" : "img/metal";

    tileImages['⬆️'] = loadImage(`${texturePath}/editor/square/top.png`);
    tileImages['⬇️'] = loadImage(`${texturePath}/editor/square/bottom.png`);
    tileImages['⬅️'] = loadImage(`${texturePath}/editor/square/left.png`);
    tileImages['➡️'] = loadImage(`${texturePath}/editor/square/right.png`);
    tileImages['↖️'] = loadImage(`${texturePath}/editor/square/top-left.png`);
    tileImages['↗️'] = loadImage(`${texturePath}/editor/square/top-right.png`);
    tileImages['↙️'] = loadImage(`${texturePath}/editor/square/bottom-left.png`);
    tileImages['↘️'] = loadImage(`${texturePath}/editor/square/bottom-right.png`);
    tileImages['⏹️'] = loadImage(`${texturePath}/editor/square/center.png`);
    tileImages['⏪'] = loadImage(`${texturePath}/editor/bottom/left.png`);
    tileImages['0️⃣'] = loadImage(`${texturePath}/editor/bottom/center.png`);
    tileImages['⏩'] = loadImage(`${texturePath}/editor/bottom/right.png`);
    tileImages['⏫'] = loadImage(`${texturePath}/editor/top/top.png`);
    tileImages['1️⃣'] = loadImage(`${texturePath}/editor/top/center.png`);
    tileImages['⏬'] = loadImage(`${texturePath}/editor/top/bottom.png`);
    tileImages['⏺️'] = loadImage(`${texturePath}/editor/single.png`);
    tileImages['2️⃣'] = loadImage(`${texturePath}/editor/corner/top-left.png`);
    tileImages['3️⃣'] = loadImage(`${texturePath}/editor/corner/top-right.png`);
    tileImages['4️⃣'] = loadImage(`${texturePath}/editor/corner/bottom-left.png`);
    tileImages['5️⃣'] = loadImage(`${texturePath}/editor/corner/bottom-right.png`);

    tileImages['👆'] = loadImage(`${texturePath}/editor/fire/up.png`);
    tileImages['👇'] = loadImage(`${texturePath}/editor/fire/down.png`);
    tileImages['👈'] = loadImage(`${texturePath}/editor/fire/left.png`);
    tileImages['👉'] = loadImage(`${texturePath}/editor/fire/right.png`);

    tileImages['🟨'] = loadImage(`img/editor/yellow/door.png`);
    tileImages['🟡'] = loadImage(`img/editor/yellow/button.png`);
    tileImages['💛'] = loadImage(`img/editor/yellow/key.png`);

    tileImages['🟥'] = loadImage(`img/editor/red/door.png`);
    tileImages['🔴'] = loadImage(`img/editor/red/button.png`);
    tileImages['❤️'] = loadImage(`img/editor/red/key.png`);
}

function setup() {
    const canvas = createCanvas(400, 400);
    canvas.parent('editor');

    for (let i = 0; i < 16; i++) {
        grid.push(Array(16).fill('⬜'));
    }
}

function draw() {
    noStroke();
    noSmooth();

    const tileSize = width / grid.length;
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const tileX = j * tileSize;
            const tileY = i * tileSize;
            const tileValue = grid[i][j];
            if (tileImages[tileValue]) image(tileImages[tileValue], tileX, tileY, tileSize, tileSize);
            else {
                fill(255);
                rect(tileX, tileY, tileSize, tileSize);
            }
        }
    }

    let gx = int((mouseX - mouseX % 25) / 25);
    let gy = int((mouseY - mouseY % 25) / 25);

    if (mouseIsPressed) {
        const row = floor(mouseY / (height / grid.length));
        const col = floor(mouseX / (width / grid[0].length));
        if (row >= 0 && row < grid.length && col >= 0 && col < grid[row].length) {
            if ((selected === '🟨' || selected === '🟥' || selected === '🟦') && (gy > 0 && grid[gy - 1][gx] !== '⬜')) return;
            else if ((gy < 15 && grid[gy + 1][gx] === '🟨') || (gy < 15 && grid[gy + 1][gx] === '🟥') || (gy < 15 && grid[gy + 1][gx] === '🟦')) return;

            grid[row][col] = eraser ? '⬜' : selected;
        }
    }

    eraser ? fill(255, 255, 255, 100) : fill(0, 0, 0, 100);
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

function ToggleTiles() {
    texturePack === "natural" ? texturePack = "metal" : texturePack = "natural";
    loadTileImages();
    changeMenu();
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
                    .replace('⏹', '⏹️')
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

function changeMenu() {
    const texturePath = texturePack === "natural" ? "./img/natural" : "./img/metal";

    document.getElementById("tiles-image").src = `${texturePath}/icons/square/top.png`;
    document.getElementById("tiles-text").innerText = texturePack === "natural" ? "Natural" : "Metal";

    document.getElementById("wall-1").src =`${texturePath}/icons/square/top.png`;
    document.getElementById("wall-2").src =`${texturePath}/icons/square/bottom.png`;
    document.getElementById("wall-3").src =`${texturePath}/icons/square/left.png`;
    document.getElementById("wall-4").src =`${texturePath}/icons/square/right.png`;
    document.getElementById("wall-5").src =`${texturePath}/icons/square/top-left.png`;
    document.getElementById("wall-6").src =`${texturePath}/icons/square/top-right.png`;
    document.getElementById("wall-7").src =`${texturePath}/icons/square/bottom-left.png`;
    document.getElementById("wall-8").src =`${texturePath}/icons/square/bottom-right.png`;
    document.getElementById("wall-9").src =`${texturePath}/icons/square/center.png`;
    document.getElementById("wall-10").src =`${texturePath}/icons/bottom/left.png`;
    document.getElementById("wall-11").src =`${texturePath}/icons/bottom/center.png`;
    document.getElementById("wall-12").src =`${texturePath}/icons/bottom/right.png`;
    document.getElementById("wall-13").src =`${texturePath}/icons/top/top.png`;
    document.getElementById("wall-14").src =`${texturePath}/icons/top/center.png`;
    document.getElementById("wall-15").src =`${texturePath}/icons/top/bottom.png`;
    document.getElementById("wall-16").src =`${texturePath}/icons/single.png`;
    document.getElementById("wall-17").src =`${texturePath}/icons/corner/top-left.png`;
    document.getElementById("wall-18").src =`${texturePath}/icons/corner/top-right.png`;
    document.getElementById("wall-19").src =`${texturePath}/icons/corner/bottom-left.png`;
    document.getElementById("wall-20").src =`${texturePath}/icons/corner/bottom-right.png`;

    document.getElementById("fire-1").src = `${texturePath}/icons/fire/up.png`;
    document.getElementById("fire-2").src = `${texturePath}/icons/fire/down.png`;
    document.getElementById("fire-3").src = `${texturePath}/icons/fire/left.png`;
    document.getElementById("fire-4").src = `${texturePath}/icons/fire/right.png`;
}