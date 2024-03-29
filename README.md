# Stellar Guesser

## Building
### Windows
To build this for Windows, simply run `cargo build` or `cargo build --release`.

### Android
To build this for Android, you have to have `cargo-mobile` installed, along with the required Android tools. To set up `cargo-mobile`, follow the steps in this [guide](https://hackmd.io/XIcEwk4GSxy8APZhSa0UnA). Then run `cargo android apk build` to build the universal apk.

If you encounter issues with building the apk, with an error saying something along the lines of `note: =armv7-a"" was unexpected at this time.`, take look at this [comment](https://github.com/android/ndk/issues/1856#issuecomment-1542248775) and edit the `armv7a-linux-androideabi24-clang.cmd` script accordingly (this is an issue when using NDK v25).

## Credits
The native events handling and window creation was generated from the [agdk-egui example](https://github.com/rust-mobile/rust-android-examples).
