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
	<a href="https://crates.io/crates/mdbook-catppuccin"><img src="https://img.shields.io/crates/v/mdbook-catppuccin?colorA=363a4f&colorB=ee99a0&style=for-the-badge"></a>
</p>

<p align="center">
	<img src="assets/catwalk.webp"/>
</p>

## Previews

<details>
<summary>🌻 Latte</summary>
<img src="assets/latte.webp"/>
</details>
<details>
<summary>🪴 Frappé</summary>
<img src="assets/frappe.webp"/>
</details>
<details>
<summary>🌺 Macchiato</summary>
<img src="assets/macchiato.webp"/>
</details>
<details>
<summary>🌿 Mocha</summary>
<img src="assets/mocha.webp"/>
</details>

## Installation

### Rust (Preferred)

1. Install `mdbook-catppuccin` by running `cargo install mdbook-catppuccin` or download the latest [release](https://github.com/catppuccin/mdBook/releases/latest).

2. Navigate to your mdBook's root directory (same location where the `book.toml`
   lives) and run `mdbook-catppuccin install`

3. Build your mdBook using `mdbook build` and enjoy your new catppuccin
   flavours!

### Manual

1. Navigate to `src/bin/assets` within this repository

2. Download all assets: `index.hbs`, `catppuccin.css` and
   `catppuccin-admonish.css`

   > [!NOTE]
   > The `catppuccin-admonish.css` file is **NOT** required if you are not using
   > the [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish) plugin.

3. Transfer downloaded assets to your mdBook's theme directory (the default
   directory is `./theme` beside `book.toml`)

4. Update `additional-css` key within the `book.toml` as shown below

   ```diff
   [output.html]
   -additional-css = []
   +additional-css = ["./theme/catppuccin.css", "./theme/catppuccin-admonish.css"]
   ```

5. Build your mdBook using `mdbook build` and enjoy your new catppuccin
   flavours!

### Development

1. Clone the repository and navigate to the repository root.

   ```shell
   git clone https://github.com/catppuccin/mdbook
   cd mdbook
   ```

2. Run the following command to generate the CSS files that will be packaged by the rust binary:

   ```shell
   npm run build
   ```

3. Build the rust binary using the following command:

   Note that the **Minimum Rust Version** is 1.63.0

   ```shell
   cargo build --release
   ```

4. Test the binary by installing the new assets on the [`example`](./example/) directory:

   ```shell
   cd example
   ../../target/release/mdbook-catppuccin install
   mdbook serve
   ```

## 🙋 FAQ

- Q: **_"What's the point of the `mdbook-catppuccin` binary?"_**\
  A: Arguably, it's better to transfer over the files manually to avoid
  installing another tool. However, the assets will **NOT** be managed for you.
  It is also worth mentioning that the binary will be able to detect differences
  in versions of assets.

- Q: **_"What's the `catppuccin-admonish.css` file?"_**\
  A: It is a CSS file that is used to style the admonishments that are generated
  by the [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish).
  **You can remove this file if you are not using this plugin.**

  E.g.
  ```diff
  [output.html]
  -additional-css = ["./theme/catppuccin.css", "./theme/catppuccin-admonish.css"]
  +additional-css = ["./theme/catppuccin.css"]
  ```

- Q: **_"How can I remove the default themes?"_**\
  A:

  - Navigate to
    [`index.hbs (L125 - L133)`](https://github.com/catppuccin/mdBook/blob/main/src/bin/assets/index.hbs#L125-L133)
    and remove the themes that you don't want. Remember to run `mdbook build` again!
  - E.g. To remove all default themes:

    ```diff
    - <li role="none"><button role="menuitem" class="theme" id="light">{{ theme_option "Light" }}</button></li>
    - <li role="none"><button role="menuitem" class="theme" id="rust">{{ theme_option "Rust" }}</button></li>
    - <li role="none"><button role="menuitem" class="theme" id="coal">{{ theme_option "Coal" }}</button></li>
    - <li role="none"><button role="menuitem" class="theme" id="navy">{{ theme_option "Navy" }}</button></li>
    - <li role="none"><button role="menuitem" class="theme" id="ayu">{{ theme_option "Ayu" }}</button></li>
    + <li role="none"><button role="menuitem" class="theme" id="latte">{{ theme_option "Latte" }}</button></li>
    + <li role="none"><button role="menuitem" class="theme" id="frappe">{{ theme_option "Frappé" }}</button></li>
    + <li role="none"><button role="menuitem" class="theme" id="macchiato">{{ theme_option "Macchiato" }}</button></li>
    + <li role="none"><button role="menuitem" class="theme" id="mocha">{{ theme_option "Mocha" }}</button></li>
    ```

## Acknowledgement

Inspiration for the `install` command came from
[mdbook-admonish](https://github.com/tommilligan/mdbook-admonish) which is
another great mdBook pre-processor!

## 💝 Thanks to

- [Hamothy](https://github.com/sgoudham)
- [winston](https://github.com/nekowinston)

&#160;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
