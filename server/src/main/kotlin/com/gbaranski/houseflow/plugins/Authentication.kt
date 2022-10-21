package com.gbaranski.houseflow.plugins

import com.gbaranski.houseflow.Credentials
import com.gbaranski.houseflow.HubID
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.plugins.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.auth.jwt.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.response.*
import io.ktor.server.request.*
import io.ktor.server.routing.*

fun Application.configureAuthentication(credentials: Credentials) {
    install(Authentication) {
        jwt("auth-jwt") {
        }
    }

    routing {
        get("/json/kotlinx-serialization") {
            call.respond(mapOf("hello" to "world"))
        }
    }
}
