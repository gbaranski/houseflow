package com.gbaranski.houseflow

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonClassDiscriminator

// TODO: Maybe swap that in favor of java.util.UUID
typealias AccessoryID = String

@Serializable
enum class AccessoryType {
    @SerialName("mijia/hygro-thermometer")
    MijiaHygrothermometer,

    @SerialName("houseflow/gate")
    HouseflowGate,
    @SerialName("houseflow/garage")
    HouseflowGarage,
    @SerialName("houseflow/lightbulb")
    HouseflowLightbulb
}

@Serializable
data class Accessory(
    val id: AccessoryID,
    val name: String,
    @SerialName("room-name")
    val roomName: String,
    val type: AccessoryType,
)

@Serializable
@JsonClassDiscriminator("name")
sealed class Service

@Serializable
@SerialName("temperature-sensor")
object TemperatureSensor: Service()

@Serializable
@SerialName("humidity-sensor")
object HumiditySensor: Service()

@Serializable
@JsonClassDiscriminator("name")
sealed class Characteristic

@Serializable
@SerialName("current-temperature")
data class CurrentTemperature(val temperature: Double? = null): Characteristic()

@Serializable
@SerialName("current-humidity")
data class CurrentHumidity(val humidity: Double? = null): Characteristic()