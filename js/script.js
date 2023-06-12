let grid = [];
let eraser = false;

function setup() {
    createCanvas(400, 400);

    for (let i = 0; i < 16; i++) {
        grid.push([])
        for (let j = 0; j < 16; j++) {
            grid[i].push('.')
        }
    }
}

function draw() {
    background(220);

    for (let i = 0; i < 16; i++) {
        for (let j = 0; j < 16; j++) {
            noStroke();

            if (grid[j][i] == '#') fill(255, 0, 0)
            else fill(255)

            rect(i * 25, j * 25, 25, 25);
        }
    }

    let gx = int((mouseX - mouseX % 25) / 25)
    let gy = int((mouseY - mouseY % 25) / 25)

    if (gx > 15) gx = 15;
    if (gx < 0) gx = 0;
    if (gy > 15) gy = 15;
    if (gy < 0) gy = 0;

    if (mouseIsPressed) {
        if (eraser) grid[gy][gx] = '.';
        else grid[gy][gx] = '#';
    }

    if (eraser) fill(255, 255, 255, 100);
    else fill(255, 0, 0, 100);

    rect(gx * 25, gy * 25, 25, 25);
}

function keyPressed() {
    if (key == 'e') {
        eraser = !eraser
        alert("Eraser " + eraser)
    }

    if (key == 's') {
        output = ""

        for (let i = 0; i < 16; i++) {
            s = ""
            for (let j = 0; j < 16; j++) {
                s += grid[i][j]
            }

            output = output + s + "\n";
            // print("\"" + s + "\",");
        }

        print(output)
        navigator.clipboard.writeText(output);
    }
}