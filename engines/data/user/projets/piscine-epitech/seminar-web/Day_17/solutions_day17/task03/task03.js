const textBox = document.querySelector("footer div");

const input = document.createElement("textarea");
textBox.appendChild(input);

const output = document.createElement("p");
textBox.appendChild(output);

input.addEventListener("input", () => {
  output.textContent = input.value.slice(-42);
});
