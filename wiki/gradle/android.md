# Android

To create a minimal Android project, setup Gradle and replace contents of `build.gradle` with[^1][^2]:

```gradle
buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath 'com.android.tools.build:gradle:8.5.2'
    }
}

repositories {
    google()
}

apply plugin: 'com.android.application'

android {
    namespace "pl.czak.minimal"

    compileSdk 34

    defaultConfig {
        targetSdk 34
    }
}
```

You can check for the latest Android plugin version [here](https://mvnrepository.com/artifact/com.android.tools.build/gradle?repo=google).

Importantly, Android plugin is not compatible with Java plugin.

## Changing paths

Paths can be change by specifying sourcesets[^3]:

```gradle
android {
    sourceSets {
        main {
            manifest {
                srcFile "./AndroidManifest.xml"
            }
            java {
                srcDir "./src/main/java/"
            }
        }
    }
}
```

You can see all sourcesets by running `./gradlew sourceSets`

## Native activity

[^1]: https://github.com/czak/minimal-android-project
[^2]: https://github.com/czak/minimal-android-project/pull/4
[^3]: https://stackoverflow.com/questions/70501858/how-can-i-specify-location-of-androidmanifest-xml
