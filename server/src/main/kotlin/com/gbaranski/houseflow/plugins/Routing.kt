package com.gbaranski.houseflow.plugins

import com.gbaranski.houseflow.*
import io.ktor.server.routing.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.html.*
import io.ktor.server.response.*
import io.ktor.server.request.*
import io.ktor.util.*
import kotlinx.coroutines.future.await
import kotlinx.html.body
import kotlinx.html.form
import kotlinx.html.h1
import kotlinx.html.title

class UnsupportedCharacteristicException: Throwable()

fun update(accessoryID: AccessoryID, serviceName: ServiceName, characteristic: Characteristic) {
    println("updating $accessoryID/$serviceName/${characteristic.name} to $characteristic")
}

fun Application.configureRouting(credentials: Credentials) {
    val accessoryStore = AccessoryStore()
    val googleSmartHomeApp = GoogleSmartHomeApp()

    routing {
        get("/health-check") {
            call.respondText("I'm alive!")
        }
        post("/accessory/{accessoryID}/connected") {
            val accessory = call.receive<Accessory>()
            val accessoryID = call.parameters["accessoryID"] !!
            accessoryStore.connected(accessoryID, accessory)
        }
        post("/accessory/{accessoryID}/disconnected") {
            val accessoryID = call.parameters["accessoryID"] !!
            accessoryStore.disconnected(accessoryID)
        }
        patch("/accessory/{accessoryID}/{service}/{characteristic}") {
            val accessoryID = call.parameters["accessoryID"] !!
            val serviceName = ServiceName.values().find{
                it.codename == call.parameters["service"] !!
            } !!
            val characteristicName = CharacteristicName.values().find{
                it.codename == call.parameters["characteristic"] !!
            } !!
            val characteristic = when(serviceName) {
                ServiceName.TemperatureSensor -> when(characteristicName) {
                    CharacteristicName.CurrentTemperature -> call.receive<CurrentTemperature>()
                    else -> {
                        throw UnsupportedCharacteristicException()
                    }
                }
                ServiceName.HumiditySensor -> when(characteristicName) {
                    CharacteristicName.CurrentHumidity -> call.receive<CurrentHumidity>()
                    else -> {
                        throw UnsupportedCharacteristicException()
                    }
                }
            }
            update(accessoryID, serviceName, characteristic)
            call.response.status(HttpStatusCode.OK)
        }
        get("/auth") {
            val clientID = call.request.queryParameters["client_id"] !!
            val redirectURI = call.request.queryParameters["redirect_uri"] !!
            val state = call.request.queryParameters["state"] !!
            val scope = call.request.queryParameters["scope"] ?: ""
            val responseType = call.request.queryParameters["response_type"] !!
            val userLocale = call.request.queryParameters["user_locale"] !!

            assert(clientID == credentials.googleClientID)
            assert(responseType == "code")

            call.respondHtml(HttpStatusCode.OK) {
                body {
                    h1 {
                        +"OAuth 2.0 Authorization"
                    }
                    form {

                    }
                }
            }

        }
        post("/google-smart-home") {
            val body = call.receiveText()
            val headers = call.request.headers.toMap()
            val response = googleSmartHomeApp.handleRequest(body, headers).await()
            call.respond(response)
        }
    }
}

