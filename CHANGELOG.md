# CHANGELOG

## [2.0.0](https://github.com/catppuccin/mdBook/compare/mdbook-catppuccin-v1.0.0...mdbook-catppuccin-v2.0.0) (2023-09-17)


### ⚠ BREAKING CHANGES

* merge css files and separate admonish

### Features

* add mdbook-admonish support ([#59](https://github.com/catppuccin/mdBook/issues/59)) ([c8ba6ec](https://github.com/catppuccin/mdBook/commit/c8ba6ec236ba0bf7f4579dd8d31f27d454d7d61c))
* dependabot is over party ([07a4d8a](https://github.com/catppuccin/mdBook/commit/07a4d8a06c6ec4b65e0ed6702bf61849acc9e073))
* Fix codeblock colours ([e93f034](https://github.com/catppuccin/mdBook/commit/e93f0347722d3bd6ce5caa81cccbc93d0b205db6))
* Generate catppuccin flavours with sass ([44144a2](https://github.com/catppuccin/mdBook/commit/44144a2512265c0eb6d1376a1da4c0be1cf43894))
* **mdbook:** Add insert_dotted_table() ([37620fe](https://github.com/catppuccin/mdBook/commit/37620fe8c375ba4eaf54ae70a086445b82bc7a1c))
* Refactor + Wrap up port ([9ba9813](https://github.com/catppuccin/mdBook/commit/9ba9813fd3d1ac75106cc2b0add0ae6627aae2c4))
* **scss:** Generate css for .hjls ([c151055](https://github.com/catppuccin/mdBook/commit/c1510551c54cc3bc1c5751a6769fdcca3531f3e4))
* style nested `blockquote` and `hr` ([f78383c](https://github.com/catppuccin/mdBook/commit/f78383ce5bedd6b276d6f12656f35f2474de91fe))
* theme editable playground ([3adcd2d](https://github.com/catppuccin/mdBook/commit/3adcd2ddbf96031dd7d761705149fa5bb6aaac4e))
* use npm package imports, update highlightjs ([f3bcc7e](https://github.com/catppuccin/mdBook/commit/f3bcc7e601fb1850d120c12e961cf8d4fb3883ed))


### Bug Fixes

* Add 'ayu' back into index.hbs ([45220f0](https://github.com/catppuccin/mdBook/commit/45220f0fcca322f5c1b3285371b84942daae3d70))
* **cargo:** add `cargo` feature for `crate_version` ([dde0c18](https://github.com/catppuccin/mdBook/commit/dde0c18b73f8742f0e5ef29629d1107552b7bab1))
* different colours for code blocks and inline code ([87a1749](https://github.com/catppuccin/mdBook/commit/87a1749fdf8d6046aa986fe5199fea74bd5299e1))
* Fix white background colour ([38514d4](https://github.com/catppuccin/mdBook/commit/38514d4f5c670d197e8c10fe07d3fff5f182ccb8))
* move output from `/bin/assets` to `/src/bin/assets` ([759ad4f](https://github.com/catppuccin/mdBook/commit/759ad4fe0e7e575214115a4eb0f64b32f9b294e8))
* Update assets and resolve highlights ([caedfd0](https://github.com/catppuccin/mdBook/commit/caedfd094ae0be4421cf4f235ab80dfdfb929c33))
* update to fixed highlightjs version ([98cf81a](https://github.com/catppuccin/mdBook/commit/98cf81a998a49c3176854f46c6e9a971f34b1c93))


### Code Refactoring

* merge css files and separate admonish ([5a160cd](https://github.com/catppuccin/mdBook/commit/5a160cd7e9a8ba21fc4cb4c884e3a664fcf715ea))

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
