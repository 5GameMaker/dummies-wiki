# libs.gradle.toml

`gradle/libs.gradle.toml` is a file where versions can be defined to be
used in the codebase, allowing to update all of them quickly:
```toml
[versions]
"version-tag" = "1.2.3"

[plugins]
my-plugin = { id = "org.my.plugin", version.ref = "version-tag" }
thatone = { id = "org.my.that", version = "3.2.1" }

[libaries]
that-lib = "org.lib:that:1.3.2"
other-lib = "org.lib:other:1.3.2"
extras = "org.lib:extras:1.3.2"

[bundles] # According to docs, bundles specifically combine libraries
stuff = ["that-lib", "other-lib"]
```

Those can be accessed like this
```groovy
plugins {
    alias(libs.plugins.my.plugin)
    alias(libs.plugins.thatone)
}

dependencies {
    implementation libs.bundles.stuff
    compileOnly libs.extras
}
```

Dashes and underscores in paths are replaced with dots.

Libraries, instead of having their own section (`libs.libraries`), are placed in root of `libs`.

## settings.gradle

Alternatively, `settings.gradle` can be used instead:
```groovy
dependencyResolutionManagement {
    versionCatalogs {
        libs {
            version("version-tag", "1.2.3")

            plugin("my-plugin", "org.my.plugin").versionRef("version-tag")
            plugin("thatone", "org.my.that").version("3.2.1")

            library("that-lib", "org.lib", "that").version("1.3.2")
            library("other-lib", "org.lib", "other").version("1.3.2")
            library("extras", "org.lib", "extras").version("1.3.2")

            bundle("stuff", ["that-lib", "other-lib", "extras"])
        }
    }
}
```

External `libs.versions.toml` files can be imported as well
```groovy
dependencyResolutionManagement {
    versionCatalogs {
        extraLibs {
            from(files("gradle/extraLibs.versions.toml"))
        }
    }
}
```

This will create `extraLibs` object.

## Reference
- [Gradle: Version Catalogs](https://docs.gradle.org/current/userguide/version_catalogs.html)
