function initLinks() {
  let links = document.getElementsByClassName("linkable");
  for (i = 0; i < links.length; i++) {
    let elem = links[i];
    if (!elem.innerText.endsWith("#")) {
      continue;
    }

    let text = elem.innerText.slice(0, -1);
    elem.innerText = text;

    let handle = document.createElement("a");
    handle.classList += "linkable-handle";
    handle.href = "#" + elem.innerText;
    handle.innerText = "#";

    // handle.onclick = function () {
    //   updateLinks();
    // };

    elem.appendChild(handle);
  }

  updateLinks();
}

function resize() {
  let current_width = (window.innerWidth - 660) / 2;
  let px_to_em = parseFloat(getComputedStyle(document.body).fontSize);

  let left_nav = document.querySelector("#left-nav");
  let right_nav = document.querySelector("#right-nav");

  if (current_width < px_to_em * 20) {
    left_nav.className = "box-left nav";
    right_nav.className = "box-right nav";
  } else {
    left_nav.className = "sidebar-left nav";
    right_nav.className = "sidebar-right nav";
  }
}

window.addEventListener("DOMContentLoaded", () => {
  initLinks();
  resize();
});

window.addEventListener("resize", () => {
  resize();
});
