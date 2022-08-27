<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin for <a href="https://rust-lang.github.io/mdBook/">mdBook</a>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
	<a href="https://github.com/catppuccin/mdBook/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/mdBook?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/mdBook/issues"><img src="https://img.shields.io/github/issues/catppuccin/mdBook?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/mdBook/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/mdBook?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<img src="assets/catwalk.webp"/>
</p>

## Installation
It is important to know that there are 2 ways to install this theme:

1. Install `mdbook-catppuccin` using [Cargo](https://doc.rust-lang.org/cargo/) (It comes installed with [Rust](https://rustup.rs/) by default!)

    - **Binaries** are also available if that's more your style! You can always find the latest releases [here](https://github.com/catppuccin/mdBook/releases/latest)!
    
2. Transfer files to your mdBook from this GitHub manually

### Rust (Preferred)

1. Install `mdbook-catppuccin` by running `cargo install mdbook-catppuccin`

2. Navigate to your mdBook's root directory (same location where the `book.toml` lives) and run `mdbook-catppuccin install`

3. Build your mdBook using `mdbook build` and enjoy your new catppuccin flavours!

### Manual

1. Navigate to `src/bin/assets` within this repository

2. Download all assets: `index.hbs`, `catppuccin.css` and `catppuccin-highlight.css`

3. Transfer downloaded assets to your mdBook theme directory (the default directory would be `theme/` from the mdBook root if not already specified within the `book.toml`)

4. Update `additional-css` key within the `book.toml` as shown below

    ```diff
    [output.html]
    -additional-css = []
    +additional-css = ["./theme/catppuccin.css", "./theme/catppuccin-highlight.css"]
    ```

5. Build your mdBook using `mdbook build` and enjoy your new catppuccin flavours!

## FAQ

### How Can I Remove The Default mdBook Themes?

Navigate to [`index.hbs (L125 - L132)`](https://github.com/catppuccin/mdBook/blob/main/src/bin/assets/index.hbs#L125-L132) and remove each theme that you do not want appearing on the mdBook

For example, to remove all the default themes:

```diff
<ul id="theme-list" class="theme-popup" aria-label="Themes" role="menu">
-    <li role="none"><button role="menuitem" class="theme" id="light">{{ theme_option "Light" }}</button></li>
-    <li role="none"><button role="menuitem" class="theme" id="rust">{{ theme_option "Rust" }}</button></li>
-    <li role="none"><button role="menuitem" class="theme" id="coal">{{ theme_option "Coal" }}</button></li>
-    <li role="none"><button role="menuitem" class="theme" id="navy">{{ theme_option "Navy" }}</button></li>
-    <li role="none"><button role="menuitem" class="theme" id="ayu">{{ theme_option "Ayu" }}</button></li>
+    <li role="none"><button role="menuitem" class="theme" id="latte">{{ theme_option "Latte" }}</button></li>
+    <li role="none"><button role="menuitem" class="theme" id="frappe">{{ theme_option "Frappé" }}</button></li>
+    <li role="none"><button role="menuitem" class="theme" id="macchiato">{{ theme_option "Macchiato" }}</button></li>
+    <li role="none"><button role="menuitem" class="theme" id="mocha">{{ theme_option "Mocha" }}</button></li>
</ul>
```

Running `mdbook build` again should get rid of the themes appearing in the theme picker, enjoy!

### What's The Point Of This mdBook Preprocessor If I Can Just Install Everything Manually?

Arguably, it's actually better to transfer over the assets individually as you won't have to install this mdBook preprocessor.
However, the assets will _**NOT**_ be managed for you. You are in charge of upgrading/modifying the assets as you see fit. The preferred method of installation is through Rust as the pre-processor will be able to detect differences in versions of assets.

## Acknowledgement

Inspiration for the `install` command came from [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish) which is another great mdBook pre-processor!

## 💝 Thanks to

-   [Hamothy](https://github.com/sgoudham)
-   [winston](https://github.com/nekowinston)

&nbsp;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
