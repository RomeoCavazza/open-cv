// Initialisation

const canvas = document.querySelector("footer > div:first-of-type canvas");
const ctx    = canvas.getContext("2d");
const output = document.querySelector("footer > div:nth-of-type(2)");

// On pointe sur le carré
const SIZE = 16;
let x = 50, y = 50;
let dragging = false, offsetX = 0, offsetY = 0;

function draw() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "black";
  ctx.fillRect(x, y, SIZE, SIZE);
  output.textContent = `New coordinates => {x:${x}, y:${y}}`;
}
draw();

// On déclare le début du drag
function dragStart(e) {
  const r = canvas.getBoundingClientRect();
  const mx = e.clientX - r.left, my = e.clientY - r.top;
  if (mx >= x && mx <= x + SIZE && my >= y && my <= y + SIZE) {
    dragging = true;
    offsetX = mx - x;
    offsetY = my - y;
  }
}

// ... le drag en mouvement
function dragMove(e) {
  if (!dragging) return;
  const r = canvas.getBoundingClientRect();
  x = e.clientX - r.left - offsetX;
  y = e.clientY - r.top  - offsetY;
  // bornes
  x = Math.max(0, Math.min(x, canvas.width  - SIZE));
  y = Math.max(0, Math.min(y, canvas.height - SIZE));
  draw();
}

// fin du drag
function dragEnd() {
  dragging = false;
}

canvas.addEventListener("mousedown", dragStart);
canvas.addEventListener("mousemove", dragMove);
canvas.addEventListener("mouseup", dragEnd);
canvas.addEventListener("mouseleave", dragEnd);
