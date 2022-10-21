package com.gbaranski.houseflow

import com.gbaranski.houseflow.plugins.configureRouting
import com.gbaranski.houseflow.plugins.configureSerialization
import io.ktor.network.tls.certificates.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.slf4j.LoggerFactory
import java.io.File


fun main() {
    val credentials = credentialsFromEnv()
    val environment = applicationEngineEnvironment {
        log = LoggerFactory.getLogger("ktor.application")
        connector {
            port = 8080
        }
        module { module(credentials) }
    }
    embeddedServer(Netty, environment).start(wait = true)
}

fun Application.module(credentials: Credentials) {
    configureRouting(credentials)
    configureSerialization()
}