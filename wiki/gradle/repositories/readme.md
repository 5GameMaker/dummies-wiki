# Repositories

## Adding a repository

Repositories are defined in the `repositories` block.

```gradle
repositories {
    mavenCentral()
}
```

Built-in repositories:
- `mavenCentral()`
- `gradlePluginPortal()`

Other repositories can be referenced with
```gradle
repositories {
    maven { url = "<url>" }
    ivy { url = "<url>" }
}
```

## Publishing

To publish an artifact, `maven-publish` or `ivy-publish` plugins must be added.

It is later configured in `publishing` block.

```gradle
plugins {
    id 'maven-publish'
}

publishing {
    publications {
        // Name of the block is the name of the publication.
        maven(MavenPublication) {
            from components.java

            // The following parameters are optional and are obtained from project properties
            // if not present.
            groupId 'org.gradle.sample' // or `<ROOT>.group`.
            artifactId 'project1-sample' // or `projectName` (either `rootProject.name` or name as specified
                                         // in `include` block in `settings.gradle`).
            version '1.1' // or `<ROOT>.version`.
        }
        ivy(IvyPublication) {
            organisation = 'org.gradle.sample'
            module = 'project1-sample'
            revision = '1.1'
            descriptor.status = 'milestone'
            descriptor.branch = 'testing'
            descriptor.extraInfo 'http://my.namespace', 'myElement', 'Some value'

            from components.java
        }
    }
}
```

If sources/javadoc jars are needed, they can be enabled with
```gradle
java {
    withJavadocJar()
    withSourcesJar()
}
```

Repositories to publish to are defined via `publishing.repositories` block.
```gradle
publishing {
    repositories {
        // 'ivy' instead of 'maven' for an ivy repository
        maven {
            url = "file://<repo-path>"
            name = "[name]" // optional
        }
    }
}
```

## Publish tasks

Publishing plugin generates publish tasks with names depending on contents of `publishing` block, taking
the form of `:publish<Publication>PublicationTo<Repository>Repository`.

i.e. for
```gradle
publishing {
    publications {
        // Name of the block is the name of the publication
        someLibrary(MavenPublication) {
            from components.java
        }
    }
    repositories {
        maven {
            url = "file://<repo-path>"
            // name defaults to 'maven'
        }
    }
}
```
The task is named `:publishSomeLibraryPublicationToMavenRepository`.

`:publish` task will run all the generated publishing tasks on the repo.

## Other useful repositories

- JitPack
[Website](https://jitpack.io/)

Depend on a GitHub repository.

```gradle
repositories {
    maven { url 'https://jitpack.io' }
}

dependencies {
    implementation 'com.github.User:Repo:Tag'
}
```

## References

- <https://stackoverflow.com/questions/24827733/how-do-you-set-the-maven-artifact-id-of-a-gradle-project/24827804#24827804>
- <https://docs.gradle.org/current/userguide/building_java_projects.html>
- <https://docs.gradle.org/current/userguide/publishing_setup.html>
- <https://docs.gradle.org/current/userguide/publishing_ivy.html>
- <https://docs.gradle.org/current/userguide/publishing_maven.html>
- <https://jitpack.io/>
