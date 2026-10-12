# Adding Kotlin to an existing project

In `build.gradle`[^1][^2]:
```gradle
buildscript {
    ext {
        kotlinVersion = "2.4.20"
    }

    repositories {
        mavenCenter()
    }

    dependencies {
        classpath "org.jetbrains.kotlin:kotlin-gradle-plugin:$kotlin_version"
    }
}

plugins {
    implementation "org.jetbrains.kotlin:kotlin-stdlib:$kotlin_version"
}
```

You can find the latest Kotlin version at <https://mvnrepository.com/artifact/org.jetbrains.kotlin/kotlin-stdlib>.

## Using version catalogs

`gradle/libs.versions.toml`
```toml
[versions]
kotlin = "2.4.20"

[libraries]
kotlin-plugin = { module = "org.jetbrains.kotlin:kotlin-gradle-plugin", version.ref = "kotlin" }
kotlin = { module = "org.jetbrains.kotlin:kotlin-stdlib", version.ref = "kotlin" }
```

`build.gradle`
```gradle
buildscript {
    repositories {
        mavenCenter()
    }

    dependencies {
        classpath libs.kotlin.plugin
    }
}

plugins {
    implementation libs.kotlin
}
```

## External Kotlin

Important! The versions of Kotlin STDs in the external library and your project **must exactly match**. Kotlin *will*
refuse to load otherwise.

With either of the examples above, move Kotlin STD dependency to `compileOnly`:
```gradle
buildscript {
    repositories {
        mavenCenter()
    }

    dependencies {
        classpath libs.kotlin.plugin
    }
}

plugins {
    compileOnly libs.kotlin
}
```

and set a flag in `gradle.properties`[^3]:
```
kotlin.stdlib.default.dependency = false
```

[^1]: https://github.com/ykode/android-kotlin/blob/master/build.gradle
[^2]: https://github.com/kakajika/kotlin-minimum/blob/master/build.gradle
[^3]: https://github.com/Darkdustry-Coders/NilPlugin/blob/master/gradle.properties
