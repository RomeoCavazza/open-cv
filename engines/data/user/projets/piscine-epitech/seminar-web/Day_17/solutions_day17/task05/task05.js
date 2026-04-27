const canvas = document.querySelector("footer > div:first-of-type canvas");
const ctx = canvas.getContext("2d");
const output = document.querySelector("footer > div:nth-of-type(2)");

ctx.beginPath();
ctx.moveTo(6, 6);
ctx.lineTo(14, 10);
ctx.lineTo(6, 14);
ctx.closePath();
ctx.fillStyle = "white";
ctx.fill();
ctx.stroke();

const audio = new Audio("https://universal-soundbank.com/sounds/1027.mp3");
canvas.addEventListener("click", () => audio.play());

const [pauseBtn, stopBtn, muteBtn] = document.querySelectorAll("footer button");
pauseBtn.addEventListener("click", () => audio.pause());
stopBtn.addEventListener("click", () => { audio.pause(); audio.currentTime = 0; });
muteBtn.addEventListener("click", () => audio.muted = !audio.muted);