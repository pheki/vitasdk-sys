# vitasdk-sys

This crate exports bindings to functions available in [vitasdk](https://vitasdk.org/) and statically links to all of its stubs libraries. Their official docs are [here](https://docs.vitasdk.org/) and the bindings are automatically generated from vitasdk's [vita-headers](https://github.com/vitasdk/vita-headers.git) repository.

To be able to use it, you need vitasdk available and the environment variable `VITASDK` set to its location. e.g.:

```
$ export VITASDK=/opt/vitasdk
```

## Manually generating the bindings

Clone the repository with submodules (the C headers):

```sh
$ git clone --recurse-submodules https://github.com/pheki/vitasdk-sys.git
```

If the repository is already cloned, update the submodules with:

```sh
$ git submodule update --init --recursive
```

Generate the bindings with:

```sh
$ cd generator
$ cargo run
```

Note that `$VITASDK` must me set, if its not, follow the instructions at https://vitasdk.org/. It's required because of the platform includes.

## Updating

To update the headers, go to the vita-headers submodule and update by the usual means:

```
$ cd generator/vita-headers
$ git pull
```

Then generate by following the procedure above. Depending on upstream changes you may need to tweak `generator/config.toml` (documented at `generator/src/config.rs`) for the new headers to work. In case you need to, feel free to contribute by opening a pull request.

## Versioning

Even though usual `semver` rules apply for this crate, I believe that `vitasdk` has some sort of versioning and we're not following it right now. We're just keeping updated with the latest version of `vita-headers` and generating bindings for them. If you need bindings for newer versions or wish to improve the current situation, feel free to open an issue.

## Credits

- [**VitaSDK team**](http://vitasdk.org/) for the toolchain, vitasdk itself, etc.
- [rust-bindgen contributors](https://github.com/rust-lang/rust-bindgen) for allowing auto generated bindings.
- [Martin Larralde](https://github.com/althonos) for [psp2-sys](https://github.com/vita-rust/psp2-sys), which I believe originally inspired me to create this crate.

## License

This crate (library) is distributed under terms of MIT license or Apache License (Version 2.0), at your option.
See `LICENSE-MIT` and `LICENSE-APACHE` for terms.
