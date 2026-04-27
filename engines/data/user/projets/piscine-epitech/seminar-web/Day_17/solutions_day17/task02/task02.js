const openDialog = document.querySelector("footer div");

openDialog.addEventListener("click", () => {
    let name = prompt("Enter a character string");
    while (!name) {
        name = prompt("What's your name?");
    }
    if (confirm("Are you sure " + name + " is your name?")) {
        alert("Hello " + name + "!");
        openDialog.textContent = "Hello " + name + "!";
    }
});