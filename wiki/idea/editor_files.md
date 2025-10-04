# Editor files

IntelliJ IDEA editor files are stored in `.idea`. Options may change version to version.

## .name
Plaintext file. Stores the name of the project as shown in UI.

## gradle.xml
Absolute paths can be substituted with `$PROJECT_DIR$`, although IDEA may not
generate that.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project version="4">
    <!-- Purpose is unknown. -->
    <component name="GradleMigrationSettings" migrationVersion="1" />
    <!-- Gradle settings. Editable via project settings. -->
    <component name="GradleSettings">
        <option name="linkedExternalProjectsSettings">
            <GradleProjectSettings>
                <!-- Gradle modules for complex projects. Populated automatically. -->
                <compositeConfiguration>
                    <compositeBuild compositeDefinitionSource="SCRIPT">
                        <builds>
                            <!-- Gradle project. Multiple tags for extra. -->
                            <build path="{project full path}" name="{project display name}">
                                <projects>
                                    <!-- Gradle module. Multiple tags for extra. -->
                                    <project path="{project module path}" />
                                </projects>
                            </build>
                        </builds>
                    </compositeBuild>
                </compositeConfiguration>
                <!-- Path to gradle used for this build. -->
                <option name="gradleHome" value="{full path to libexec}" />
                <!--
                    Version of JDK used for this build.

                    TODO: Check what values this accepts.
                -->
                <option name="gradleJvm" value="17" />
                <!-- Purpose is unknown. -->
                <option name="externalProjectPath" value="$PROJECT_DIR$" />
                <!-- List of all gradle modules. -->
                <option name="modules">
                    <set>
                        <!-- More for more gradle modules. Populated automatically. -->
                        <option value="{module path}" />
                    </set>
                </option>
            </GradleProjectSettings>
        </option>
    </component>
</project>
```

## Sources

- <https://intellij-support.jetbrains.com/hc/en-us/community/posts/4408459384594-gradle-xml-gradleHome-How-is-this-populated>
