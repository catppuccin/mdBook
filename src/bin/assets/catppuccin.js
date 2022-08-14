var flavours = ["Latte", "Frappe", "Macchiato", "Mocha"];
var theme_list = document.getElementById("theme-list");

for (let flavour of flavours) {
    var theme = document.createElement("li");
    theme.setAttribute("role", "none");
    theme.innerHTML = `<button role="menuitem" class="theme" id="${flavour.toLowerCase()}">${flavour}</button>`;
    theme_list.appendChild(theme);
}
