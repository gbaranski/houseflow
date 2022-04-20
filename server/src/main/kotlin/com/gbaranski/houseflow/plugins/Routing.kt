package com.gbaranski.houseflow.plugins

import io.ktor.server.routing.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.request.*

fun Application.configureRouting() {

    routing {
        get("/health-check") {
            call.respondText("I'm alive!")
        }
    }
}
