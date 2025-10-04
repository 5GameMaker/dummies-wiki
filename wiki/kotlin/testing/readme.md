# Kotlin Testing

## Setup

Add stuff

- `build.gradle`
```groovy
def kotlinVersion = ".." // i.e. 2.1.21

plugins {
    id 'org.jetbrains.kotlin.plugin.power-assert' version "$kotlinVersion"
}

dependencies {
    testImplementation "org.jetbrains.kotlin:kotlin-test"
}

test {
    useJUnitPlatform()
    testLogging {
        events "skipped", "failed"

        showExceptions true
        exceptionFormat "full"
        showCauses true
        showStackTraces true

        showStandardStreams = false
    }
}

powerAssert {
    functions = ["kotlin.assert", "kotlin.test.assertTrue", "kotlin.test.assertEquals", "kotlin.test.assertNull"]
    includedSourceSets = ["main", "test"]
}
```

- `src/test/kotlin/Test.kt` 
```kotlin
import kotlin.test.Test

class Tests {
    @Test
    fun someTest() {
        assert(1 == 1)
    }
}
```

## Always test on build

If you're fine waiting some extra time for tests to succeed, add this to `build.gradle`:
```groovy
tasks.named("build") {
    dependsOn ":test"
}
```

## Reference

- [Kotlin Docs Page](https://kotlinlang.org/docs/jvm-test-using-junit.html)
- [A random thread I took test configuration from](https://discuss.gradle.org/t/how-do-i-get-more-details-about-failed-test-assertions-on-the-shell/29495)
- [Power assert](https://kotlinlang.org/docs/power-assert.html)
