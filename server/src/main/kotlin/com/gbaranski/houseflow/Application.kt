package com.gbaranski.houseflow

import io.ktor.server.engine.*
import io.ktor.server.netty.*
import com.gbaranski.houseflow.plugins.*

fun main() {
    embeddedServer(Netty, port = 8080, host = "0.0.0.0") {
        configureRouting()
        configureSecurity()
        configureSerialization()
        configureSockets()
    }.start(wait = true)
}
