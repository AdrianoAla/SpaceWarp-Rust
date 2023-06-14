let grid = [];
let eraser = false;
let currentColor = [136, 139, 153];

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

            if (grid[j][i] == '#') fill(136, 139, 153)
            else if (grid[j][i] == '!') fill(255, 154, 0)
            else if (grid[j][i] == '(') fill(255, 255, 0)
            else if (grid[j][i] == '<') fill(240, 230, 140)
            else if (grid[j][i] == '-') fill(128, 128, 0)
            else if (grid[j][i] == '[') fill(255, 0, 0)
            else if (grid[j][i] == '>') fill(220, 20, 60)
            else if (grid[j][i] == '=') fill(139, 0, 0)
            else if (grid[j][i] == '{') fill(0, 0, 255)
            else if (grid[j][i] == '^') fill(0, 191, 255)
            else if (grid[j][i] == '+') fill(25, 25, 112)
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
        else {
            if (currentColor.join(",") === "136,139,153") grid[gy][gx] = '#';
            else if (currentColor.join(",") === "255,154,0") grid[gy][gx] = '!';
            else if (currentColor.join(",") === "255,255,0") grid[gy][gx] = '(';
            else if (currentColor.join(",") === "240,230,140") grid[gy][gx] = '<';
            else if (currentColor.join(",") === "128,128,0") grid[gy][gx] = '-';
            else if (currentColor.join(",") === "255,0,0") grid[gy][gx] = '[';
            else if (currentColor.join(",") === "220,20,60") grid[gy][gx] = '>';
            else if (currentColor.join(",") === "139,0,0") grid[gy][gx] = '=';
            else if (currentColor.join(",") === "0,0,255") grid[gy][gx] = '{';
            else if (currentColor.join(",") === "0,191,255") grid[gy][gx] = '^';
            else if (currentColor.join(",") === "25,25,112") grid[gy][gx] = '+';
        }
    }

    if (eraser) fill(255, 255, 255, 100);
    else fill(currentColor[0], currentColor[1], currentColor[2], 100);

    rect(gx * 25, gy * 25, 25, 25);
}

function keyPressed() {
    if (key == '1') {
        currentColor = [136, 139, 153];
        alert("Walls")
    }

    if (key == '2') {
        currentColor = [255, 154, 0]
        alert("Fire")
    }

    if (key == '3') {
        currentColor = [255, 255, 0]
        alert("Yellow door")
    }

    if (key == '4') {
        currentColor = [240, 230, 140]
        alert("Yellow button")
    }

    if (key == '5') {
        currentColor = [128, 128, 0]
        alert("Yellow key")
    }

    if (key == '6') {
        currentColor = [255, 0, 0]
        alert("Red door")
    }

    if (key == '7') {
        currentColor = [220, 20, 60]
        alert("Red button")
    }

    if (key == '8') {
        currentColor = [139, 0, 0]
        alert("Red key")
    }

    if (key == '9') {
        currentColor = [0, 0, 255]
        alert("Blue door")
    }

    if (key == '0') {
        currentColor = [0, 191, 255]
        alert("Blue button")
    }

    if (key == '-') {
        currentColor = [25, 25, 112]
        alert("Blue key")
    }

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