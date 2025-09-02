plugins {
    id("com.android.application")
    id("rust")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace="com.github.noreply.users.stellar_guesser"
    compileSdk = 35
    defaultConfig {
        applicationId = "com.github.noreply.users.stellar_guesser"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "1.0"
    }
    sourceSets.getByName("main") {
        // Vulkan validation layers
        val ndkHome = System.getenv("NDK_HOME")
        jniLibs.srcDir("${ndkHome}/sources/third_party/vulkan/src/build-android/jniLibs")
    }
    buildTypes {
        getByName("debug") {
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
             proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
}

rust {
    rootDirRel = "../../"
}

dependencies {
    implementation("com.google.android.material:material:1.8.0")
}
