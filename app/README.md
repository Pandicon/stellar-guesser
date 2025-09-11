# Stellar Guesser

## Building
### Windows
To build this for Windows, simply run `cargo build` or `cargo build --release`.

### Android
To build this for Android, you have to have `cargo-mobile` installed, along with the required Android tools. To set up `cargo-mobile`, follow the steps in this [guide](https://hackmd.io/XIcEwk4GSxy8APZhSa0UnA). Then run `cargo android apk build` to build the debug universal apk.
You will also need to have a reasonable version of Android SDK installed (can be done via Android Studio), as of September 2025 that is v35 and/or v34 (see [#62](/../../issues/62)). Along with that you should install Java 17 (Kotlin and Gradle do not support newer versions) from the [Oracle website](https://www.oracle.com/java/technologies/javase/jdk17-archive-downloads.html). Then set the `JAVA_HOME` environment variable to the location of the JDK (on Windows it is usually `C:\Program Files\Java\jdk-<version>`, you can see it during installation). Ensure you do not have a newer version installed at all and that there are no newer folders in the Java folder, else it may fall back to that (even if only an empty folder exists, for example having an empty `C:\Program Files\Java\jdk-24` folder next to `C:\Program Files\Java\jdk-17.0.2` did not work for me).

If you encounter issues with building the apk, with an error saying something along the lines of `note: =armv7-a"" was unexpected at this time.`, take look at this [comment](https://github.com/android/ndk/issues/1856#issuecomment-1542248775) and edit the `armv7a-linux-androideabi24-clang.cmd` script accordingly (this is an issue when using NDK v25).

To build the release version, you have to create a keystore with a signing key (see [step 1 here](https://stackoverflow.com/a/40064199)). Name the keystore `stellar-guesser.keystore` and place it to the root of the project (next to this file). Then you can run the `build-android-release-and-sign.bat` batch script from the `build-scripts` folder to fully build the release apk, align it, and sign it. You will have to add `%ANDROID_SDK_ROOT%\cmdline-tools\latest\bin`, `%ANDROID_SDK_ROOT%\platform-tools`, and `%ANDROID_SDK_ROOT%\build-tools\<your version>` to the `PATH` environment variable (assuming you also have an environment varibale callted `ANDROID_SDK_ROOT` set to the installation location of the Android SDK, usually `C:\Users\<username>\AppData\Android\Sdk` on Windows).

To change the target Android SDK version and the version the app is built for, go to `./gen/android/app/` and find the `build.gradle.kts` file. Within the `android` object, there is the `defaultConfig` attribute containing `targetSdk`, which specifies which SDK version the build targets and was tested on. There is also the `compileSdk` attribute which specifies which Android SDK version the application will be built for (has to be >= `targetSDK`) and so dictates which SDK features are accessible in the app. For more details see [this StackOverflow answer](https://stackoverflow.com/a/47269079).

To change the app id or version, go to `./gen/android/app/` and find the `build.gradle.kts` file. Then in the `android` "object", change the `namespace` and `applicationId` to the desired application id, `versionName` to the desired version name (like `1.2.1`), and also increase `versionCode` if you are releasing an update (this is what Google Play uses to serve updates, not `versionName`). IMPORTANT: If you change the app id, change it in `./src/public_constants.rs` too, else files saving won't work. (TODO: Link these together, see [#8](/../../issues/8))

To change the app icon, go to `./gen/android/app/src/main/res/` and change out the icon in each of the `mipmap-...` folders. Also delete/move out the `drawable`, `drawable-v24`, and `mipmap-anydpi-v26` folders (the ones with xml files in them), or find a way to replace those too.

Also in `./gen/android/app/src/main/`, see the `AndroidManifest.xml` file and add in a line containing `<uses-permission android:name="android.permission.INTERNET"/>` - this way the app will be able to access the internet. It should go inside the `manifest` tag, but outside the `application` tag.

## Testing
Apart from the usual way of testing the app by using it, one can also enable some additional testing UI by setting the `TESTING` environmental variable to `true`. This can be done for example by having a `.env` file next to the binary and have a `TESTING=true` line in it. This additional UI lets us test things by hand, for example one can highlight stars inside a given constellation etc. - this would be very difficult to make a unit test for and it is probably safer to test it visually first.

## Debugging
### Android
When debugging the Android app, look for the following strings in logs:
 - `RustStdoutStderr`
 - `stellar_guesser` (the logs emitted by the app through `log`)
 - `panic`

## Credits
The native events handling and window creation was generated from the [agdk-egui example](https://github.com/rust-mobile/rust-android-examples).
