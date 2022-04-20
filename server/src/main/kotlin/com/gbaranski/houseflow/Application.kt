package com.gbaranski.houseflow

import com.gbaranski.houseflow.plugins.configureRouting
import com.gbaranski.houseflow.plugins.configureSerialization
import com.gbaranski.houseflow.plugins.configureSockets
import io.ktor.server.engine.*
import io.ktor.server.netty.*


fun main() {
    embeddedServer(Netty, port = 8080, host = "0.0.0.0") {
        configureRouting()
        configureSerialization()
        configureSockets()
    }.start(wait = true)
}
