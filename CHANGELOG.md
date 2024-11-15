# CHANGELOG

## [3.1.0](https://github.com/catppuccin/mdBook/compare/v3.0.3...v3.1.0) (2024-11-15)


### Features

* add support for `lambdalisue/rs-mdbook-alerts` ([#135](https://github.com/catppuccin/mdBook/issues/135)) ([cd3ca47](https://github.com/catppuccin/mdBook/commit/cd3ca47e761df31288641a5134879b8e76af827f))


### Bug Fixes

* change inline codeblocks from `peach` to `text` ([45da623](https://github.com/catppuccin/mdBook/commit/45da62372ca500102ec542cebc39ec999f196ff2))
* increase contrast on icons hover ([#140](https://github.com/catppuccin/mdBook/issues/140)) ([7edf128](https://github.com/catppuccin/mdBook/commit/7edf128b50f9a256d3a0ec37defad3ff755369fa))
* preserve links as `blue` inside inline codeblocks ([45da623](https://github.com/catppuccin/mdBook/commit/45da62372ca500102ec542cebc39ec999f196ff2))


### Code Refactoring

* consistently use "flavor" and "color" spellings, update sass syntax ([#138](https://github.com/catppuccin/mdBook/issues/138)) ([fa0912e](https://github.com/catppuccin/mdBook/commit/fa0912e7eea8863278d566e33b2dd45d9625fd40))
* **website:** set default to `latte` ([#141](https://github.com/catppuccin/mdBook/issues/141)) ([e8a0135](https://github.com/catppuccin/mdBook/commit/e8a013513c882e390f2694f74cede5da76a5cf83))

## [3.0.3](https://github.com/catppuccin/mdBook/compare/v3.0.2...v3.0.3) (2024-09-22)


### Bug Fixes

* highlightjs syntax highlighting ([#119](https://github.com/catppuccin/mdBook/issues/119)) ([5a04755](https://github.com/catppuccin/mdBook/commit/5a04755004cf75aff86f911e684b915c9505f76f))
* properly theme `hr` element ([#122](https://github.com/catppuccin/mdBook/issues/122)) ([90d4392](https://github.com/catppuccin/mdBook/commit/90d4392a45244cde7f0c846c5c75cfa91e0349ea))
* reduce harshness of hover highlight ([ac84329](https://github.com/catppuccin/mdBook/commit/ac84329f1e251055f007a93cdff4caab732680fb))
* theme tooltip when copying from codeblock ([ac84329](https://github.com/catppuccin/mdBook/commit/ac84329f1e251055f007a93cdff4caab732680fb))

## [3.0.2](https://github.com/catppuccin/mdBook/compare/v3.0.1...v3.0.2) (2024-09-22)


### Code Refactoring

* add symlinks back ([0aad6a2](https://github.com/catppuccin/mdBook/commit/0aad6a2f148faf7f8d112a53c0529f8a80a4de80))
* **example:** remove symlinks to see if website ci passes ([291ceb7](https://github.com/catppuccin/mdBook/commit/291ceb76911b0656544c1e4b4e0cdf6322a52835))
* **example:** update mdbook-admonish & rework pages ci ([#116](https://github.com/catppuccin/mdBook/issues/116)) ([db0d22c](https://github.com/catppuccin/mdBook/commit/db0d22cc63d4d24cd23c6635de1ab55ef2f1d499))
* ignore css in example directory ([ccc7de7](https://github.com/catppuccin/mdBook/commit/ccc7de7d6c1fbb2a64da6d4cf516d30524b40afa))
* switch borders from `crust` to `surface0` ([#118](https://github.com/catppuccin/mdBook/issues/118)) ([c315046](https://github.com/catppuccin/mdBook/commit/c3150466e94cdd08e1bea5b92c18d0a7ecb84f28))

## [3.0.1](https://github.com/catppuccin/mdBook/compare/v3.0.0...v3.0.1) (2024-09-22)


### Code Refactoring

* remove all rust ([#114](https://github.com/catppuccin/mdBook/issues/114)) ([e1da91c](https://github.com/catppuccin/mdBook/commit/e1da91c95e99cf5642a4aff020865a8ffa7ca481))

## [3.0.0](https://github.com/catppuccin/mdBook/compare/v2.2.0...v3.0.0) (2024-09-22)


### ⚠ BREAKING CHANGES

* deprecate rust plugin & update sass files ([#112](https://github.com/catppuccin/mdBook/issues/112))

### Features

* deprecate rust plugin & update sass files ([#112](https://github.com/catppuccin/mdBook/issues/112)) ([f705cb5](https://github.com/catppuccin/mdBook/commit/f705cb5a0ca7932ea88480b1793b915f47b52146))

## [2.2.0](https://github.com/catppuccin/mdBook/compare/v2.1.0...v2.2.0) (2024-08-15)


### Features

* add deprecation notice ([#109](https://github.com/catppuccin/mdBook/issues/109)) ([d023fd5](https://github.com/catppuccin/mdBook/commit/d023fd52c6d31fe6236339afe1fd73d76d7f83e9))


### Build system & distribution

* don't make draft releases ([#110](https://github.com/catppuccin/mdBook/issues/110)) ([80fefcd](https://github.com/catppuccin/mdBook/commit/80fefcd3e51bf41ae1de3803e0963afb8d4b0b58))
* **nix:** fix darwin devShell ([#94](https://github.com/catppuccin/mdBook/issues/94)) ([218116e](https://github.com/catppuccin/mdBook/commit/218116e83688f2786af71eef5d11108f71950987))

## [2.1.0](https://github.com/catppuccin/mdBook/compare/v2.0.1...v2.1.0) (2023-11-11)


### Features

* **cli:** add shell completion ([a9a79c6](https://github.com/catppuccin/mdBook/commit/a9a79c6f5441f1f62ae4658d7d1aaf9f79b3b9d3))
* style `.warning` class ([#91](https://github.com/catppuccin/mdBook/issues/91)) ([ff9851d](https://github.com/catppuccin/mdBook/commit/ff9851d395934d8b5a084b7290d992b127169d51))


### Build system & distribution

* **cargo:** reduce build-time dependencies ([8e9ea6e](https://github.com/catppuccin/mdBook/commit/8e9ea6e2e9fd7b33fb62740eca4d7fb47aa3b38e))
* **cargo:** update/unpin clap and mdbook ([b209ae7](https://github.com/catppuccin/mdBook/commit/b209ae7ffa85e5cc3dba1dbf5e73b2560ad592c7))
* **distribution:** remove `linux-musl`, `win-gnu`, and `win32-msvc` ([170419d](https://github.com/catppuccin/mdBook/commit/170419dd6059a164b46ef60b85d9fc153603eb53))
* **nix:** add shell completions ([#89](https://github.com/catppuccin/mdBook/issues/89)) ([097a72a](https://github.com/catppuccin/mdBook/commit/097a72ab17272063772f88441c902511f36bd188))
* **nix:** drop macOS native dependencies ([#88](https://github.com/catppuccin/mdBook/issues/88)) ([547ed85](https://github.com/catppuccin/mdBook/commit/547ed8546b1870cb1e5747a2f2338aa33b3af522))
* **website:** use mdbook-admonish v1.13.0 ([33eb09c](https://github.com/catppuccin/mdBook/commit/33eb09c677719474e8726e6bf7278e1c9390cacc))

## [2.0.1](https://github.com/catppuccin/mdBook/compare/v2.0.0...v2.0.1) (2023-10-09)


### Bug Fixes

* **admonish:** prefix `admonish-` to class names ([#76](https://github.com/catppuccin/mdBook/issues/76)) ([ba711b6](https://github.com/catppuccin/mdBook/commit/ba711b6ceb67f9d4e352cafc4a11d8a28d0b4980))

## [2.0.0](https://github.com/catppuccin/mdBook/compare/v1.2.0...v2.0.0) (2023-10-07)


### ⚠ BREAKING CHANGES

* support mdbook version `0.4.35`

### Features

* **cli:** add "--force" flag to install command ([28f9d0f](https://github.com/catppuccin/mdBook/commit/28f9d0f02aea41c7278659fb22ba9db5bbf02b5d))
* **cli:** detect differences in major version ([8956c61](https://github.com/catppuccin/mdBook/commit/8956c61a727b0408be202c9c7fc6c2f336166272))
* support mdbook version `0.4.35` ([55e8722](https://github.com/catppuccin/mdBook/commit/55e872267de2c63e91659c6be5eac1be4fc34540))

## [1.2.0](https://github.com/catppuccin/mdBook/compare/v1.1.0...v1.2.0) (2023-10-03)


### Features

* darken struckout text ([5388763](https://github.com/catppuccin/mdBook/commit/5388763d108a72e1f0bd99656d7c88439fe9243a))
* switch to clap v4 ([a1ddb9a](https://github.com/catppuccin/mdBook/commit/a1ddb9a11b6a5e83b75dbfeebe776ad7658ac9eb))


### Bug Fixes

* clap typecasting panic ([1193c91](https://github.com/catppuccin/mdBook/commit/1193c91b092da7f7496fb78431045733814789f0))
* colours in `diff` codeblock ([#67](https://github.com/catppuccin/mdBook/issues/67)) ([a3b3f77](https://github.com/catppuccin/mdBook/commit/a3b3f7726c85cc815279f62fbd096d7249429866))

## [1.1.0](https://github.com/catppuccin/mdBook/compare/v1.0.0...v1.1.0) (2023-10-02)


### Features

* darken struckout text ([5388763](https://github.com/catppuccin/mdBook/commit/5388763d108a72e1f0bd99656d7c88439fe9243a))
* switch to clap v4 ([a1ddb9a](https://github.com/catppuccin/mdBook/commit/a1ddb9a11b6a5e83b75dbfeebe776ad7658ac9eb))


### Bug Fixes

* colours in `diff` codeblock ([#67](https://github.com/catppuccin/mdBook/issues/67)) ([a3b3f77](https://github.com/catppuccin/mdBook/commit/a3b3f7726c85cc815279f62fbd096d7249429866))

## [1.0.0](https://github.com/catppuccin/mdBook/compare/mdbook-catppuccin-v0.2.1...mdbook-catppuccin-v1.0.0) (2023-09-17)


### ⚠ BREAKING CHANGES

* merge css files and separate admonish

### Features

* add mdbook-admonish support ([#59](https://github.com/catppuccin/mdBook/issues/59)) ([c8ba6ec](https://github.com/catppuccin/mdBook/commit/c8ba6ec236ba0bf7f4579dd8d31f27d454d7d61c))
* style nested `blockquote` and `hr` ([f78383c](https://github.com/catppuccin/mdBook/commit/f78383ce5bedd6b276d6f12656f35f2474de91fe))

### Code Refactoring

* merge css files and separate admonish ([5a160cd](https://github.com/catppuccin/mdBook/commit/5a160cd7e9a8ba21fc4cb4c884e3a664fcf715ea))

## [v0.2.1](https://github.com/catppuccin/mdBook/releases/tag/v0.2.1) - 2023-07-29

### Bug Fixes

* add `cargo` feature for `crate_version` by @sgoudham (dde0c18b73f8742f0e5ef29629d1107552b7bab1)
	* This fixes #52  
* pin mdbook to 0.4.22 by @nyxkrage (cc92a344d1693d6f0a54fc55110af57bb2dfe70a)
	* This will prevent issues like #52 from happening again. 
* Fix source filter in `flake.nix` by @VojtechStep in https://github.com/catppuccin/mdBook/pull/51

## [v0.2.0](https://github.com/catppuccin/mdBook/releases/tag/v0.2.0) - 2023-06-16 02:13:13

Apologies for leaving this on the backburner for so long!

This release makes the port more maintainable as we are now using [catppuccin/highlightjs](https://github.com/catppuccin/highlightjs) for the majority of the CSS.
There has also been improvements to the CI/CD pipeline and includes a Nix flake (which I still need to fix!)

Thanks to anyone who chooses to use this, I really appreciate it <3

### Feature

- general:
  - use npm package imports, update highlightjs ([f3bcc7e](https://github.com/catppuccin/mdBook/commit/f3bcc7e601fb1850d120c12e961cf8d4fb3883ed)) ([#39](https://github.com/catppuccin/mdBook/pull/39))
  - dependabot is over party ([07a4d8a](https://github.com/catppuccin/mdBook/commit/07a4d8a06c6ec4b65e0ed6702bf61849acc9e073)) ([#16](https://github.com/catppuccin/mdBook/pull/16))

### Bug Fixes

- general:
  - different colours for code blocks and inline code ([87a1749](https://github.com/catppuccin/mdBook/commit/87a1749fdf8d6046aa986fe5199fea74bd5299e1))
  - code backgrounds updated to `mantle` ([5feb4df](https://github.com/catppuccin/mdBook/commit/5feb4df45897b9399351588e230960ec9cdeefda)) ([#45](https://github.com/catppuccin/mdBook/pull/45))
  - update to fixed highlightjs version ([98cf81a](https://github.com/catppuccin/mdBook/commit/98cf81a998a49c3176854f46c6e9a971f34b1c93)) ([#39](https://github.com/catppuccin/mdBook/pull/39))
  - move output from `/bin/assets` to `/src/bin/assets` ([759ad4f](https://github.com/catppuccin/mdBook/commit/759ad4fe0e7e575214115a4eb0f64b32f9b294e8)) ([#39](https://github.com/catppuccin/mdBook/pull/39))

### Documentation

- general:
  - add example mdBook ([3469e9e](https://github.com/catppuccin/mdBook/commit/3469e9ee873fa395396f374a33b3a0f1b5c5d5b4)) ([#45](https://github.com/catppuccin/mdBook/pull/45))

- README:
  - standardise with template repository ([3844bd1](https://github.com/catppuccin/mdBook/commit/3844bd10008bd5dabc8fba5ecf3b691c6830daba)) ([#45](https://github.com/catppuccin/mdBook/pull/45))

### Refactor

- general:
  - links should be `blue` ([6cce7d0](https://github.com/catppuccin/mdBook/commit/6cce7d0264b0e813974ce8b3967c249ed90a34ab)) ([#45](https://github.com/catppuccin/mdBook/pull/45))

## [v0.1.1](https://github.com/catppuccin/mdBook/releases/tag/v0.1.1) - 2022-08-27 21:05:04

_No description_

### Bug Fixes

- general:
  - Add 'ayu' back into index.hbs ([45220f0](https://github.com/catppuccin/mdBook/commit/45220f0fcca322f5c1b3285371b84942daae3d70)) ([#1](https://github.com/catppuccin/mdBook/pull/1))

### Documentation

- README:
  - Add ayu into diff codeblock example ([b68dac3](https://github.com/catppuccin/mdBook/commit/b68dac3c77ac55c4467ba2e1c8746774a12445a8)) ([#1](https://github.com/catppuccin/mdBook/pull/1))
  - Add binaries section ([9f7bae8](https://github.com/catppuccin/mdBook/commit/9f7bae8045f1312d1cc20decf8780c02325e98f1))
  - Update link for index.hbs ([200008e](https://github.com/catppuccin/mdBook/commit/200008e7b237ffe2b7895a6792bded9ed30cd335))

## [v0.1.0](https://github.com/catppuccin/mdBook/releases/tag/v0.1.0) - 2022-08-27 12:47:37

🎉 The first-ever release of mdbook-catppuccin 🎉

![image](https://user-images.githubusercontent.com/58985301/187031293-f3a931f1-cf14-4d47-820b-5185288b0969.png)

### Feature

- general:

  - Refactor + Wrap up port ([9ba9813](https://github.com/catppuccin/mdBook/commit/9ba9813fd3d1ac75106cc2b0add0ae6627aae2c4))
  - Fix codeblock colours ([e93f034](https://github.com/catppuccin/mdBook/commit/e93f0347722d3bd6ce5caa81cccbc93d0b205db6))
  - Generate catppuccin flavours with sass ([44144a2](https://github.com/catppuccin/mdBook/commit/44144a2512265c0eb6d1376a1da4c0be1cf43894))

- mdbook:

  - Add insert_dotted_table() ([37620fe](https://github.com/catppuccin/mdBook/commit/37620fe8c375ba4eaf54ae70a086445b82bc7a1c))

- scss:
  - Generate css for .hjls ([c151055](https://github.com/catppuccin/mdBook/commit/c1510551c54cc3bc1c5751a6769fdcca3531f3e4))

### Bug Fixes

- general:
  - Fix white background colour ([38514d4](https://github.com/catppuccin/mdBook/commit/38514d4f5c670d197e8c10fe07d3fff5f182ccb8))
  - Update assets and resolve highlights ([caedfd0](https://github.com/catppuccin/mdBook/commit/caedfd094ae0be4421cf4f235ab80dfdfb929c33))

### Documentation

- README:

  - Tidy up diff codeblock ([5bd3618](https://github.com/catppuccin/mdBook/commit/5bd3618abc00b3b306d87952621b7a43cbb52908))
  - Update badge links ([9ab04ba](https://github.com/catppuccin/mdBook/commit/9ab04ba20b3ecafd832753f0371a9ed22ecd1b28))
  - Import template from catppuccin ([00b5f47](https://github.com/catppuccin/mdBook/commit/00b5f4765c6e84d3ab80d383d922f2299fad88a3))

- general:
  - Update description of crate ([686e801](https://github.com/catppuccin/mdBook/commit/686e80150cb382e58e7ebe90d574843d062cf879))
  - Add binaries bullet point ([c960264](https://github.com/catppuccin/mdBook/commit/c960264bf5eabff96e6c797defa62e8ab67506e8))
  - Update README & upload catwalk image ([1b945c5](https://github.com/catppuccin/mdBook/commit/1b945c50313311500987a543cb3d7d2f70da0c32))

### Refactor

- general:

  - No need for individual images ([af14a54](https://github.com/catppuccin/mdBook/commit/af14a542bae658a58673741cf762f7841fb266b6))
  - Remove useless comment ([db3cf56](https://github.com/catppuccin/mdBook/commit/db3cf5607bafb25873d50c8b993512f286c10ddb))
  - Remove unused code ([bc01543](https://github.com/catppuccin/mdBook/commit/bc015432546dcf99695b10edd2333d6fd8d6fe25))
  - Remove old commented css ([56b47b3](https://github.com/catppuccin/mdBook/commit/56b47b39cf6980b68739a3496b56b2a329a966c8))

- assets:

  - Create separate file for highlights ([a69a8de](https://github.com/catppuccin/mdBook/commit/a69a8de064ccbc10344e3ae30785270cc094c42e))
  - Move src/assets -> src/bin/assets ([444b809](https://github.com/catppuccin/mdBook/commit/444b8096454bf04b2a973766994703523e6c1d3d))
  - Remove ASSETS_VERSION in favour of CARGO_PKG_VERSION ([5384c92](https://github.com/catppuccin/mdBook/commit/5384c929e99a3ce35e28b821419c6f4bb74c4b2f))

- mdbook:

  - Extend default toml structs ([48cd282](https://github.com/catppuccin/mdBook/commit/48cd282227ee2de3a2f691176e6c8d81159217f2))
  - Move toml trait extensions into toml.rs ([cbfffa3](https://github.com/catppuccin/mdBook/commit/cbfffa3c944b1934ddac65c50ee8b792b438e4b6))
  - Start highlighting codeblocks + other stuff ([c24d08a](https://github.com/catppuccin/mdBook/commit/c24d08abc6ceb4f6f89fcf6385e52df7a92ce367))

- config:
  - Use spaces instead of tabs ([3c4d7e0](https://github.com/catppuccin/mdBook/commit/3c4d7e0b1ffc24644fdefbe4b1ca8c344590b87a))

\* _This CHANGELOG was automatically generated by [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog)_
