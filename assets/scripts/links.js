function initLinks() {
  let links = document.getElementsByClassName("linkable");
  for (i = 0; i < links.length; i++) {
    let elem = links[i];
    if (!elem.innerText.endsWith("#")) {
      continue;
    }

    let text = elem.innerText.slice(0, -1);
    elem.innerText = text;

    if (window.location.hash.slice(1) == text) {
      elem.classList.add("linkable-active");
    }

    let handle = document.createElement("a");
    handle.classList += "linkable-handle";
    handle.href = "#" + elem.innerText;
    handle.innerText = "#";

    elem.appendChild(handle);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  initLinks();
});
