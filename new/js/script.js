let grid = [];
let eraser = false;
let selected = "⬆️";
let selectedElement = "recent-1";
let tileImages = {};
let texturePack = "metal";
let menuCheck = false;
let recent = [
    "⬆️",
    "⬇️",
    "⬅️",
    "➡️"
];

let texturePath = `img/${texturePack}`;

function preload() {
    loadTileImages();
}

function loadTileImages() {
    const imagePaths = [
        ['⬆️', '/square/top.png'],
        ['⬇️', '/square/bottom.png'],
        ['⬅️', '/square/left.png'],
        ['➡️', '/square/right.png'],
        ['↖️', '/square/top-left.png'],
        ['↗️', '/square/top-right.png'],
        ['↙️', '/square/bottom-left.png'],
        ['↘️', '/square/bottom-right.png'],
        ['⏹️', '/square/center.png'],
        ['⏪', '/bottom/left.png'],
        ['0️⃣', '/bottom/center.png'],
        ['⏩', '/bottom/right.png'],
        ['⏫', '/top/top.png'],
        ['1️⃣', '/top/center.png'],
        ['⏬', '/top/bottom.png'],
        ['⏺️', '/single.png'],
        ['2️⃣', '/corner/top-left.png'],
        ['3️⃣', '/corner/top-right.png'],
        ['4️⃣', '/corner/bottom-left.png'],
        ['5️⃣', '/corner/bottom-right.png'],
        ['👆', '/fire/up.png'],
        ['👇', '/fire/down.png'],
        ['👈', '/fire/left.png'],
        ['👉', '/fire/right.png'],
        ['🟨', '/objects/yellow/door.png'],
        ['🟡', '/objects/yellow/button.png'],
        ['💛', '/objects/yellow/key.png'],
        ['🟥', '/objects/red/door.png'],
        ['🔴', '/objects/red/button.png'],
        ['❤️', '/objects/red/key.png'],
        ['🟦', '/objects/blue/door.png'],
        ['🔵', '/objects/blue/button.png'],
        ['💙', '/objects/blue/key.png']
    ];

    for (const [tileValue, imagePath] of imagePaths) {
        tileImages[tileValue] = loadImage(`${texturePath}${imagePath}`);
    }
}

function setup() {
    const canvas = createCanvas(800, 800);
    canvas.parent('editor');

    background(255);

    grid = Array.from({ length: 16 }, () => Array(16).fill('⬜'));
}

function draw() {
    background(255);

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

    const gx = int((mouseX - mouseX % 25) / 25);
    const gy = int((mouseY - mouseY % 25) / 25);

    if (mouseIsPressed && !menuCheck) {
        const row = floor(mouseY / (height / grid.length));
        const col = floor(mouseX / (width / grid[0].length));

        if (isValidCell(row, col)) {
            if ((selected === '🟨' || selected === '🟥' || selected === '🟦') && (gy > 0 && grid[gy - 1][gx] !== '⬜')) return;
            else if ((gy < 15 && grid[gy + 1][gx] === '🟨') || (gy < 15 && grid[gy + 1][gx] === '🟥') || (gy < 15 && grid[gy + 1][gx] === '🟦')) return;

            grid[row][col] = eraser ? '⬜' : selected;
        }
    }

    if (!menuCheck) {
        fill(eraser ? 255 : 0, eraser ? 255 : 0, eraser ? 255 : 0, 100);
        rect(gx * 25, gy * 25, 50, 50);
    }
}

function isValidCell(row, col) {
    return row >= 0 && row < grid.length && col >= 0 && col < grid[row].length;
}

function keyPressed() {
    if (key === 'e') toggleEraser();
}

function setObject(value, id) {
    selected = !isNaN(value) ? recent[value] : value;

    document.getElementById(selectedElement).classList.remove('selected');

    selectedElement = id;

    if (isNaN(value)) {
        if (!recent.includes(value)) recent.unshift(value);
        else selectedElement = `recent-${recent.indexOf(value) + 1}`;

        if (recent.length > 4) recent.pop();
    }

    refreshMenu();
    setSelected();
}

function setSelected() {
    document.getElementById(selectedElement).classList.add('selected');
}

function refreshMenu() {
    const paths = recent.map((element) => getPath(element));

    setMultipleElements("recent-", 4, (i) => paths[i - 1]);
}

function ToggleTiles() {
    texturePack = texturePack === "natural" ? "metal" : "natural";
    texturePath = `img/${texturePack}`;

    loadTileImages();
    changePack();
}

function toggleEraser() {
    eraser ? document.getElementById("eraser").classList.remove("selected") : document.getElementById("eraser").classList.add("selected");

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
                console.log(char)

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
                    .replace('⏺', '⏺️')
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
        const row = grid[i].join('');
        output += row + '\n';
    }

    // const spawnX = document.getElementById('spawn-x').value;
    // const spawnY = document.getElementById('spawn-y').value;
    // if (spawnX && spawnY) output += '-1\n-1\n-1\n' + spawnX + '\n' + spawnY;

    console.log(output);
    navigator.clipboard.writeText(output);
}

function changePack() {
    setElement("tiles", `./img/menu/${texturePack}.png`);

    refreshMenu();
    setMultipleElements("wall-", 20, (i) => `${texturePath}/${getImageName(i)}.png`);
    setMultipleElements("default-wall-", 4, (i) => `${texturePath}/${getImageName(i)}.png`);
    setMultipleElements("fire-", 4, (i) => `${texturePath}/fire/${getFireImageName(i)}.png`);
    setMultipleElements("default-fire-", 4, (i) => `${texturePath}/fire/${getFireImageName(i)}.png`);
    setMultipleElements("yellow-", 3, (i) => `${texturePath}/objects/yellow/${getObjectImageName(i)}.png`);
    setMultipleElements("red-", 3, (i) => `${texturePath}/objects/red/${getObjectImageName(i)}.png`);
    setMultipleElements("blue-", 3, (i) => `${texturePath}/objects/blue/${getObjectImageName(i)}.png`);
}

function setElement(elementId, imagePath) {
    (document.getElementById(elementId).querySelector("img")).src = imagePath;
}

function setMultipleElements(elementPrefix, count, imagePathFn) {
    for (let i = 1; i <= count; i++) {
        setElement(`${elementPrefix}${i}`, imagePathFn(i));
    }
}

function getImageName(index) {
    const imageNames = [
        'square/top', 'square/bottom', 'square/left', 'square/right', 'square/top-left',
        'square/top-right', 'square/bottom-left', 'square/bottom-right', 'square/center',
        'bottom/left', 'bottom/center', 'bottom/right', 'top/top', 'top/center',
        'top/bottom', 'single', 'corner/top-left', 'corner/top-right', 'corner/bottom-left',
        'corner/bottom-right'
    ];
    return imageNames[index - 1] || '';
}

function getFireImageName(index) {
    const fireImageNames = ['up', 'down', 'left', 'right'];
    return fireImageNames[index - 1] || '';
}

function getObjectImageName(index) {
    const objectImageNames = ['door', 'button', 'key'];
    return objectImageNames[index - 1] || '';
}

function getPath(value) {
    const paths = {
        "⬆️": "square/top",
        "⬇️": "square/bottom",
        "⬅️": "square/left",
        "➡️": "square/right",
        "↖️": "square/top-left",
        "↗️": "square/top-right",
        "↙️": "square/bottom-left",
        "↘️": "square/bottom-right",
        "⏹️": "square/center",
        "⏪": "bottom/left",
        "0️⃣": "bottom/center",
        "⏩": "bottom/right",
        "⏫": "top/top",
        "1️⃣": "top/center",
        "⏬": "top/bottom",
        "⏺️": "single",
        "2️⃣": "corner/top-left",
        "3️⃣": "corner/top-right",
        "4️⃣": "corner/bottom-left",
        "5️⃣": "corner/bottom-right",
        "👆": "fire/up",
        "👇": "fire/down",
        "👈": "fire/left",
        "👉": "fire/right",
        "🟨": "objects/yellow/door",
        "🟡": "objects/yellow/button",
        "💛": "objects/yellow/key",
        "🟥": "objects/red/door",
        "🔴": "objects/red/button",
        "❤️": "objects/red/key",
        "🟦": "objects/blue/door",
        "🔵": "objects/blue/button",
        "💙": "objects/blue/key"
    }

    return `${texturePath}/${paths[value]}.png`;
}

function toggleMenu() {
    const menuButton = document.getElementById("menuButton");
    const menu = document.getElementById("menu");

    if (menuCheck) {
        setElement("menuButton", "./img/menu/down.png");
        menuButton.classList.remove("selected");
        menu.style.display = "none";
        menuCheck = false;
    } else {
        setElement("menuButton", "./img/menu/up.png");
        menuButton.classList.add("selected");
        menu.style.display = "block";
        menuCheck = true;
    }
}

refreshMenu();