// Driftloom's Android app: one module, `app`. The repositories are Google's
// and Maven Central only (F-Droid builds from them, and nothing else is
// trusted); Gradle itself is the wrapper's, pinned and checksummed
// (gradle/wrapper/gradle-wrapper.properties).
pluginManagement {
    repositories {
        google {
            content {
                includeGroupByRegex("com\\.android.*")
                includeGroupByRegex("com\\.google.*")
                includeGroupByRegex("androidx.*")
            }
        }
        mavenCentral()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google {
            content {
                includeGroupByRegex("com\\.android.*")
                includeGroupByRegex("com\\.google.*")
                includeGroupByRegex("androidx.*")
            }
        }
        mavenCentral()
    }
}

rootProject.name = "driftloom-android"
include(":app")
