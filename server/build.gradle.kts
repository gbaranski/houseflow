val ktor_version: String by project
val kotlin_version: String by project
val logback_version: String by project
val tcnative_version: String by project
val google_actions_version: String by project
val myndocs_oauth_version: String by project

plugins {
    application
    kotlin("jvm") version "1.6.20"
    id("org.jetbrains.kotlin.plugin.serialization") version "1.6.20"
}

group = "com.gbaranski.houseflow"
version = "0.0.1"
application {
    mainClass.set("com.gbaranski.houseflow.ApplicationKt")

    val isDevelopment: Boolean = project.ext.has("development")
    applicationDefaultJvmArgs = listOf("-Dio.ktor.development=$isDevelopment")
}

repositories {
    mavenCentral()
    maven { url = uri("https://maven.pkg.jetbrains.space/public/p/ktor/eap") }
}

val osName = System.getProperty("os.name").toLowerCase()
val tcnative_classifier = when {
    osName.contains("win") -> "windows-x86_64"
    osName.contains("linux") -> "linux-x86_64"
    osName.contains("mac") -> "osx-x86_64"
    else -> null
}

dependencies {
    implementation("io.ktor:ktor-server-core-jvm:$ktor_version")
    implementation("io.ktor:ktor-server-auth:$ktor_version")
    implementation("io.ktor:ktor-server-auth-jvm:$ktor_version")
    implementation("io.ktor:ktor-server-auth-jwt-jvm:$ktor_version")
    implementation("io.ktor:ktor-server-content-negotiation-jvm:$ktor_version")
    implementation("io.ktor:ktor-server-html-builder:$ktor_version")
    implementation("io.ktor:ktor-serialization-kotlinx-json:$ktor_version")
    implementation("io.ktor:ktor-serialization-kotlinx-json-jvm:$ktor_version")
    implementation("io.ktor:ktor-server-netty-jvm:$ktor_version")
    implementation("io.ktor:ktor-network-tls-certificates:$ktor_version")

    implementation("ch.qos.logback:logback-classic:$logback_version")
    implementation(group = "com.google.actions", name = "actions-on-google", version = google_actions_version)

    implementation("io.netty:netty-tcnative:$tcnative_version")
    if (tcnative_classifier != null) {
        implementation("io.netty:netty-tcnative-boringssl-static:$tcnative_version:$tcnative_classifier")
    } else {
        implementation("io.netty:netty-tcnative-boringssl-static:$tcnative_version")
    }

    testImplementation("io.ktor:ktor-server-tests-jvm:$ktor_version")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlin_version")

    implementation("nl.myndocs:oauth2-server-core:$myndocs_oauth_version")
    implementation("nl.myndocs:oauth2-server-ktor:$myndocs_oauth_version")
    // In memory dependencies
    implementation("nl.myndocs:oauth2-server-client-inmemory:$myndocs_oauth_version")
    implementation("nl.myndocs:oauth2-server-identity-inmemory:$myndocs_oauth_version")
    implementation("nl.myndocs:oauth2-server-token-store-inmemory:$myndocs_oauth_version")


}

tasks {
    compileKotlin {
        kotlinOptions.jvmTarget = "18"
    }
    compileTestKotlin {
        kotlinOptions.jvmTarget = "18"
    }
}