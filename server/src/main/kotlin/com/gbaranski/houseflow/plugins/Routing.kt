package com.gbaranski.houseflow.plugins

import com.gbaranski.houseflow.*
import io.ktor.server.routing.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.request.*

class UnsupportedCharacteristicException: Throwable()

fun update(accessoryID: AccessoryID, serviceName: ServiceName, characteristic: Characteristic) {
    println("updating $accessoryID/$serviceName/${characteristic.name} to $characteristic")
}

fun Application.configureRouting() {
    val connectedAccessories = mutableMapOf<AccessoryID, Accessory>()

    routing {
        get("/health-check") {
            call.respondText("I'm alive!")
        }
        post("/accessory/{accessoryID}/connected") {
            val accessory = call.receive<Accessory>()
            val accessoryID = call.parameters["accessoryID"] !!
            connectedAccessories[accessoryID] = accessory
        }
        post("/accessory/{accessoryID}/disconnected") {
            val accessoryID = call.parameters["accessoryID"] !!
            connectedAccessories.remove(accessoryID)
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
    }
}

