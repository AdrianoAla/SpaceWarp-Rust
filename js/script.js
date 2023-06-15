let grid = [];
let eraser = false;
let selected = "⬛"
let tileImages = {};

function preload() {
    tileImages['⬛'] = loadImage('img/black_tile.png');
    tileImages['⬆️'] = loadImage('img/fire/up.png');
    tileImages['⬅️'] = loadImage('img/fire/left.png');
    tileImages['➡️'] = loadImage('img/fire/right.png');
    tileImages['⬇️'] = loadImage('img/fire/down.png');
    tileImages['🟨'] = loadImage('img/yellow/door.png');
    tileImages['🟡'] = loadImage('img/yellow/button.png');
    tileImages['💛'] = loadImage('img/yellow/key.png');
    tileImages['🟥'] = loadImage('img/red/door.png');
    tileImages['🔴'] = loadImage('img/red/button.png');
    tileImages['❤️'] = loadImage('img/red/key.png');
    tileImages['🟦'] = loadImage('img/blue/door.png');
    tileImages['🔵'] = loadImage('img/blue/button.png');
    tileImages['💙'] = loadImage('img/blue/key.png');

}

function setup() {
    createCanvas(400, 400);

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

    if (gx > 15) gx = 15;
    if (gx < 0) gx = 0;
    if (gy > 15) gy = 15;
    if (gy < 0) gy = 0;

    if (mouseIsPressed) {
        if (eraser) grid[gy][gx] = '⬜';
        else {
            if (selected === '⬛') grid[gy][gx] = '⬛';
            else if (selected === '⬆️') grid[gy][gx] = '⬆️';
            else if (selected === '⬅️') grid[gy][gx] = '⬅️';
            else if (selected === '➡️') grid[gy][gx] = '➡️';
            else if (selected === '⬇️') grid[gy][gx] = '⬇️';
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
    else fill(255, 255, 255, 100);

    rect(gx * 25, gy * 25, 25, 25);
}

function keyPressed() {
    if (key == 'r' && ['⬆️', '⬇️', '⬅️', '➡️'].includes(selected)) {
        if (selected === '⬆️') selected = '⬇️';
        else if (selected === '⬇️') selected = '⬅️';
        else if (selected === '⬅️') selected = '➡️';
        else if (selected === '➡️') selected = '⬆️';
    }

    if (key == '1') {
        selected = '⬛';
        alert('Walls');
    }

    if (key == '2') {
        selected = '⬆️';
        alert('Fire');
    }

    if (key == '3') {
        selected = '🟨';
        alert('Yellow door');
    }

    if (key == '4') {
        selected = '🟡';
        alert('Yellow button');
    }

    if (key == '5') {
        selected = '💛';
        alert('Yellow key');
    }

    if (key == '6') {
        selected = '🟥';
        alert('Red door');
    }

    if (key == '7') {
        selected = '🔴';
        alert('Red button');
    }

    if (key == '8') {
        selected = '❤️';
        alert('Red key');
    }

    if (key == '9') {
        selected = '🟦';
        alert('Blue door');
    }

    if (key == '0') {
        selected = '🔵';
        alert('Blue button');
    }

    if (key == '-') {
        selected = '💙';
        alert('Blue key');
    }

    if (key == 'e') {
        eraser = !eraser;
        alert('Eraser ' + eraser);
    }

    if (key == 's') {
        output = '';

        for (let i = 0; i < 16; i++) {
            s = '';
            for (let j = 0; j < 16; j++) {
                s += grid[i][j];
            }

            output = output + s + '\n';
        }

        print(output);
        navigator.clipboard.writeText(output);
    }

    if (key == 'i') {
        navigator.clipboard
            .readText()
            .then((clipboardData) => {
                const clipboardText = (clipboardData.toString())
                    .replace(/⬆️/g, "⬆")
                    .replace(/⬅️/g, "⬅")
                    .replace(/➡️/g, "➡")
                    .replace(/⬇️/g, "⬇");

                const rows = clipboardText.split('\n');
                // if (rows.length !== 16) return console.log('Invalid clipboard data: Expected 16 rows.');

                for (let i = 0; i < 16; i++) {
                    const row = rows[i];
                    // if (row.length !== 16) return console.log('Invalid clipboard data: Each row should have 16 characters.');

                    for (let j = 0; j < 16; j++) {
                        console.log(row.charAt(j))
                        grid[i][j] = row.charAt(j) == '⬇' ? '⬇️' : row.charAt(j);
                    }
                }

                console.log('Data imported from clipboard successfully.');
            })
            .catch((error) => {
                console.log('Failed to read clipboard data:', error);
            });
    }
}