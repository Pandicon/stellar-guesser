# Stellar Guesser

## Building
### Windows
To build this for Windows, simply run `cargo build` or `cargo build --release`.

### Android
To build this for Android, you have to have `cargo-mobile` installed, along with the required Android tools. To set up `cargo-mobile`, follow the steps in this [guide](https://hackmd.io/XIcEwk4GSxy8APZhSa0UnA). Then run `cargo android apk build` to build the debug universal apk.

If you encounter issues with building the apk, with an error saying something along the lines of `note: =armv7-a"" was unexpected at this time.`, take look at this [comment](https://github.com/android/ndk/issues/1856#issuecomment-1542248775) and edit the `armv7a-linux-androideabi24-clang.cmd` script accordingly (this is an issue when using NDK v25).

To build the release version, you have to create a keystore with a signing key (see [step 1 here](https://stackoverflow.com/a/40064199)). Name the keystore `stellar-guesser.keystore` and place it to the root of the project (next to this file). Then you can run the `build-android-release-and-sign.bat` batch script from the `build-scripts` folder to fully build the release apk, align it, and sign it.

To change the app id or version, go to `./gen/android/app/` and find the `build.gradle.kts` file. Then in the `android` "object", change the `namespace` and `applicationId` to the desired application id, `versionName` to the desired version name (like `1.2.1`), and also increase `versionCode` if you are releasing an update (this is what Google Play uses to serve updates, not `versionName`). IMPORTANT: If you change the app id, change it in `./src/public_constants.rs` too, else files saving won't work. (TODO: Link these together)

To change the app icon, go to `./gen/android/app/src/main/res/` and change out the icon in each of the `mipmap-...` folders. Also delete/move out the `drawable`, `drawable-v24`, and `mipmap-anydpi-v26` folders (the ones with xml files in them), or find a way to replace those too.

Also in `./gen/android/app/src/main/`, see the `AndroidManifest.xml` file and add in a line containing `<uses-permission android:name="android.permission.INTERNET"/>` - this way the app will be able to access the internet. It should go inside the `manifest` tag, but outside the `application` tag.

## Credits
The native events handling and window creation was generated from the [agdk-egui example](https://github.com/rust-mobile/rust-android-examples).