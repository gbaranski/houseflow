package com.gbaranski.houseflow.plugins

import com.gbaranski.houseflow.HubID
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.plugins.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.response.*
import io.ktor.server.request.*
import io.ktor.server.routing.*

fun Application.configureAuthentication() {
    install(Authentication) {
        basic("hub-auth") {
            realm = "Access to Lighthouse"
            validate { credentials ->
                // TODO: Validate the credentials
                UserIdPrincipal(credentials.name)
            }
        }
    }

    routing {
        get("/json/kotlinx-serialization") {
            call.respond(mapOf("hello" to "world"))
        }
    }
}
