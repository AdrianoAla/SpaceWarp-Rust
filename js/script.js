let grid = [];
let eraser = false;
let selected = "⬆️";
let selectedElement = "recent-1";
let tileImages = {};
let texturePack = "metal";
let menuCheck = false;
let recent = ["⬆️", "⬇️", "⬅️", "➡️", "⏹️", "👆", "👇", "👈", "👉", "🟨", "🟡", "💛"];
let texturePath = `img/${texturePack}`;

const tiles = [
    {
        "emoji": "⬆️",
        "image": "/square/top.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⬇️",
        "image": "/square/bottom.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⬅️",
        "image": "/square/left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "➡️",
        "image": "/square/right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "↖️",
        "image": "/square/top-left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "↗️",
        "image": "/square/top-right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "↙️",
        "image": "/square/bottom-left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "↘️",
        "image": "/square/bottom-right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⏹️",
        "image": "/square/center.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⏪",
        "image": "/bottom/left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "0️⃣",
        "image": "/bottom/center.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⏩",
        "image": "/bottom/right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⏫",
        "image": "/top/top.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "1️⃣",
        "image": "/top/center.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⏬",
        "image": "/top/bottom.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "⏺️",
        "image": "/single.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "2️⃣",
        "image": "/corner/top-left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "3️⃣",
        "image": "/corner/top-right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "4️⃣",
        "image": "/corner/bottom-left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "5️⃣",
        "image": "/corner/bottom-right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "👆",
        "image": "/fire/up.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "👇",
        "image": "/fire/down.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "👈",
        "image": "/fire/left.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "👉",
        "image": "/fire/right.png",
        "type": "terrain",
        "isDoor": false
    },
    {
        "emoji": "🟨",
        "image": "/objects/yellow/door/bottom.png",
        "type": "gizmos",
        "isDoor": true
    },
    {
        "emoji": "🟡",
        "image": "/objects/yellow/button.png",
        "type": "gizmos",
        "isDoor": false
    },
    {
        "emoji": "💛",
        "image": "/objects/yellow/key.png",
        "type": "gizmos",
        "isDoor": false
    },
    {
        "emoji": "🟥",
        "image": "/objects/red/door/bottom.png",
        "type": "gizmos",
        "isDoor": true
    },
    {
        "emoji": "🔴",
        "image": "/objects/red/button.png",
        "type": "gizmos",
        "isDoor": false
    },
    {
        "emoji": "❤️",
        "image": "/objects/red/key.png",
        "type": "gizmos",
        "isDoor": false
    },
    {
        "emoji": "🟦",
        "image": "/objects/blue/door/bottom.png",
        "type": "gizmos",
        "isDoor": true
    },
    {
        "emoji": "🔵",
        "image": "/objects/blue/button.png",
        "type": "gizmos",
        "isDoor": false
    },
    {
        "emoji": "💙",
        "image": "/objects/blue/key.png",
        "type": "gizmos",
        "isDoor": false
    }
]

function preload() {
    loadTileImages();
}

function loadTileImages() {
    for (let index = 0; index < tiles.length; index++) {
        const { emoji, image, type, isDoor } = tiles[index];

        tileImages[emoji] = loadImage(`${texturePath}${image}`);

        if (isDoor) tileImages[`${emoji}⬇️`] = loadImage(`${texturePath}${image.replace("bottom", "top")}`);
    }

    backgroundImage = loadImage("img/game.png");
}

function setup() {
    const canvas = createCanvas(768, 768);
    canvas.parent('editor');

    grid = Array.from({ length: 16 }, () => Array(16).fill('⬜'));
}

function draw() {
    background(backgroundImage);

    noStroke();
    noSmooth();

    const tileSize = width / grid.length;
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const tileX = j * tileSize;
            const tileY = i * tileSize;
            const tileValue = grid[i][j];
            if (tileImages[tileValue]) {
                if (isDoor(tileValue)) image(tileImages[`${tileValue}⬇️`], tileX, tileY - tileSize, tileSize, tileSize);
                image(tileImages[tileValue], tileX, tileY, tileSize, tileSize);
            }
        }
    }

    const gx = int((mouseX - mouseX % 48) / 48);
    const gy = int((mouseY - mouseY % 48) / 48);

    if (mouseIsPressed && !menuCheck) {
        const row = floor(mouseY / (height / grid.length));
        const col = floor(mouseX / (width / grid[0].length));

        if (isValidCell(row, col)) {
            if (isDoor(selected) && (gy > 0 && grid[gy - 1][gx] !== '⬜')) return;
            else if ((gy < 15 && grid[gy + 1][gx] === '🟨') || (gy < 15 && grid[gy + 1][gx] === '🟥') || (gy < 15 && grid[gy + 1][gx] === '🟦')) return;

            grid[row][col] = eraser ? '⬜' : selected;
        }
    }

    if (!menuCheck) {
        fill(eraser ? 255 : 0, eraser ? 255 : 0, eraser ? 255 : 0, 100);
        rect(gx * 48, gy * 48, 48, 48);
    }
}

function isValidCell(row, col) {
    return row >= 0 && row < grid.length && col >= 0 && col < grid[row].length;
}

function isDoor(emoji) {
    const tile = tiles.find((tile) => tile.emoji === emoji);
    return tile && tile.isDoor;
}

function isSymbolPlaced(symbol) {
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            if (grid[i][j] === symbol) return true;
        }
    }
    return false;
}

function setObject(value, id) {
    selected = !isNaN(value) ? recent[value] : value;

    document.getElementById(selectedElement).classList.remove('selected');

    selectedElement = id;

    if (isNaN(value)) {
        if (!recent.includes(value)) recent.unshift(value);
        else selectedElement = `recent-${recent.indexOf(value) + 1}`;

        if (recent.length > 12) recent.pop();
    }

    refreshMenu();
    setSelected();
}

function setSelected() {
    document.getElementById(selectedElement).classList.add('selected');
}

function refreshMenu() {
    const paths = recent.map((element) => getPath(element));

    setMultipleElements("recent-", 12, (i) => {
        const imagePath = paths[i - 1];
        const element = document.getElementById(`recent-${i}`);

        element.className = "button";

        const tile = tiles.find((tile) => `img/${texturePack}${tile.image}` === imagePath);
        if (tile) element.classList.add(tile.type);

        return imagePath;
    });
}

function toggleEraser() {
    eraser ? document.getElementById("eraser").classList.remove("selected") : document.getElementById("eraser").classList.add("selected");

    eraser = !eraser;
}

function importRoom() {
    if (navigator.clipboard && navigator.clipboard.readText) {
        navigator.clipboard
            .readText()
            .then((clipboardData) => {
                processData(clipboardData);
            })
            .catch((error) => {
                console.log('Failed to read clipboard data:', error);
            });
    } else {
        const clipboardData = window.prompt('Please enter the data:');
        processData(clipboardData);
    }
}

function getSurrogatePairArray(str) {
    const arr = [];
    let index = 0;

    while (index < str.length) {
        const char = str.charAt(index);
        const charCode = char.charCodeAt(0);

        if (charCode >= 0xd800 && charCode <= 0xdbff) {
            const highSurrogate = charCode;
            const lowSurrogate = str.charCodeAt(index + 1);

            if (lowSurrogate >= 0xdc00 && lowSurrogate <= 0xdfff) {
                const pair = String.fromCharCode(highSurrogate, lowSurrogate);
                arr.push(pair);
                index += 2;
                continue;
            }
        }

        arr.push(char);
        index++;
    }

    return arr;
}

function processData(clipboardData) {
    const rows = [];

    if (clipboardData) {
        const clipboardClean = Array.from(clipboardData.toString())
            .filter(c => c !== '\u{fe0f}' && c !== '\u{20e3}')
            .join('');

        const clipboardText = clipboardClean.toString().replace(/\s/g, '');
        const chunkSize = 16;

        const graphemes = getSurrogatePairArray(clipboardText);
        let currentIndex = 0;

        while (currentIndex < graphemes.length) {
            const row = graphemes.slice(currentIndex, currentIndex + chunkSize);

            if (row.length < chunkSize) rows.push(row.concat(Array(chunkSize - row.length).fill('⬜')));
            else rows.push(row);

            currentIndex += chunkSize;
        }
    }

    for (let i = 0; i < 16; i++) {
        const row = rows[i] || ''.padEnd(16, '⬜');

        for (let j = 0; j < 16; j++) {
            const char = row[j];
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

    console.log('Data imported successfully.');
}

function exportRoom() {
    let output = '';

    for (let i = 0; i < 16; i++) {
        const row = grid[i].join('');
        output += row + '\n';
    }

    console.log(output);
    navigator.clipboard.writeText(output);
}

function changePack() {
    refreshMenu();
    setMultipleElements("wall-", 20, (i) => getPath(tiles[i - 1].emoji));
    setMultipleElements("fire-", 4, (i) => getPath(tiles[i + 15].emoji));
    setMultipleElements("yellow-", 3, (i) => getPath(tiles[i + 25].emoji));
    setMultipleElements("red-", 3, (i) => getPath(tiles[i + 28].emoji));
    setMultipleElements("blue-", 3, (i) => getPath(tiles[i + 31].emoji));
}

function setElement(elementId, imagePath) {
    document.getElementById(elementId).querySelector("img").src = imagePath;
}

function setMultipleElements(elementPrefix, count, imagePathFn) {
    for (let i = 1; i <= count; i++) setElement(`${elementPrefix}${i}`, imagePathFn(i));
}

function getImageName(index) {
    if (index <= tiles.length) return tiles[index - 1].image;
    return '';
}

function getFireImageName(index) {
    const fireImageNames = ['up', 'down', 'left', 'right'];
    if (index <= fireImageNames.length) return `fire/${fireImageNames[index - 1]}.png`;
    return '';
}

function getObjectImageName(index) {
    const objectImageNames = ['door/bottom', 'button', 'key'];
    if (index <= objectImageNames.length) return `objects/${objectImageNames[index - 1]}.png`;
    return '';
}

function getPath(value) {
    const tile = tiles.find((tile) => tile.emoji === value);
    if (tile) return `${texturePath}${tile.image}`;
    return '';
}

function toggleMenu() {
    const menuButton = document.getElementById("menuButton");
    const menu = document.getElementById("menu");

    if (menuCheck) {
        setElement("menuButton", "./img/menu/down.png");
        menuButton.classList.remove("selected");
        menu.style.display = "none";
    } else {
        setElement("menuButton", "./img/menu/up.png");
        menuButton.classList.add("selected");
        menu.style.display = "block";
    }

    menuCheck = !menuCheck;
}

document.addEventListener('keydown', (event) => {
    if (event.code === 'KeyE') toggleEraser();
});

const customSelect = document.querySelector('.dropdown');
const selectSelected = customSelect.querySelector('.select-selected');
const selectItems = customSelect.querySelector('.select-items');
const optionItems = customSelect.querySelectorAll('.select-items div');

customSelect.addEventListener('mouseover', () => selectItems.classList.add('open'));
customSelect.addEventListener('mouseout', () => selectItems.classList.remove('open'));

function changeMenu(option) {
    const selectedImageSrc = option.querySelector('img').src;
    const selectedHTML = option.innerHTML;

    selectSelected.innerHTML = selectedHTML;
    option.parentNode.removeChild(option);

    const ids = ["metal", "natural", "classic"];
    ids.forEach((id) => {
        if (id === option.id) return;
        document.getElementById(id).style.display = "block";
    });

    const hiddenOption = document.createElement('div');
    hiddenOption.style.display = 'none';
    hiddenOption.innerHTML = selectedHTML;
    hiddenOption.id = option.id;
    hiddenOption.onclick = () => changeMenu(hiddenOption);
    hiddenOption.querySelector('img').src = selectedImageSrc;

    selectItems.insertBefore(hiddenOption, selectItems.firstChild);
    selectItems.classList.remove('open');

    texturePath = `img/${hiddenOption.id}`;

    loadTileImages();
    changePack();
}

window.addEventListener('click', (event) => {
    if (!customSelect.contains(event.target)) selectItems.classList.remove('open');
});

// refreshMenu();