function setCookie(name, value, days) {
  const date = new Date();
  date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
  document.cookie = name + "=" + value + ";expires=" + date.toUTCString() + ";path=/";
}

function getCookie(name) {
  const ca = document.cookie.split(';');
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i].trim();
    if (c.indexOf(name + "=") === 0) return c.substring(name.length + 1);
  }
  return null;
}

function deleteCookie(name) {
  document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 UTC;path=/";
}

const firstBox = document.querySelector("footer > div");
const footer = document.querySelector("footer");
const okLink = firstBox.querySelector("a");

function createDeleteBox() {
  const deleteBox = document.createElement("div");
  deleteBox.innerHTML = 'You have accepted cookies. <button>Delete the cookie</button>';
  deleteBox.style.minHeight = "60px";
  deleteBox.style.backgroundColor = "#fff";
  deleteBox.style.textAlign = "center";
  deleteBox.style.padding = "22px";
  deleteBox.style.marginTop = "20px";
  footer.appendChild(deleteBox);
  
  deleteBox.querySelector("button").addEventListener("click", function() {
    deleteCookie("acceptsCookies");
    deleteBox.remove();
    firstBox.style.display = "block";
  });
}

if (getCookie("acceptsCookies") === "true") {
  firstBox.style.display = "none";
  createDeleteBox();
}

okLink.addEventListener("click", function(e) {
  e.preventDefault();
  setCookie("acceptsCookies", "true", 1);
  firstBox.style.display = "none";
  createDeleteBox();
});
