let clicks = 0;

const target = document.querySelector("footer div");

const counter = document.createElement("span");
target.appendChild(counter);

function onClick() {
  clicks += 1;
  counter.textContent = clicks;
}
target.addEventListener("click", onClick);