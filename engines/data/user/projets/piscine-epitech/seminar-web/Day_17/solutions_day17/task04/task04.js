let size = 16;

const buttons = document.querySelectorAll("footer div button");
buttons.forEach((btn) => {
  btn.addEventListener("click", () => {
    if (btn.textContent === "+") {
      size += 2;
    } else {
      size -= 2;
    }
    document.body.style.fontSize = size + "px";
  });
});

const select = document.querySelector("footer div select");
select.addEventListener("change", () => {
  document.body.style.backgroundColor = select.value;
});