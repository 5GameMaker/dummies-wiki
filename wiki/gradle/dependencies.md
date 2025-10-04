# Dependencies

Dependencies are defined in the `dependencies` block. `repositories` block must be present
to download dependencies.

```gradle
dependencies {
    // Included with the library (see 'Bundling dependencies').
    implementation 'some.group:Artifact:version'
    // Only present in compile time and is not included with the library.
    compileOnly 'some.group:Artifact:version'
    // Includes an annotation processor
    annotationProcessor 'some.group:Artifact:version'

    // Testing only

    // Included with the library when testing (`:test` task).
    testImplementation 'some.group:Artifact:version'
    // Only present in compile time in testing source sets.
    testCompileOnly 'some.group:Artifact:version'

    // Extras

    // (with Kotlin KSP) include a KSP processor.
    ksp 'some.group:Artifact:version'
}
```

## Bundling dependencies

Add to `build.gradle`:
```groovy
jar {
    from {
        configurations.runtimeClasspath.collect { it.isDirectory() ? it : zipTree(it) }
    }
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
}
```

This will include all dependencies marked `implementation` in the jar.
