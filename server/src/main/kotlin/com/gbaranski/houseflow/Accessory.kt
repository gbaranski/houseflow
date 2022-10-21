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
    MijiaHygroThermometer,

    @SerialName("houseflow/gate")
    HouseflowGate,
    @SerialName("houseflow/garage")
    HouseflowGarage,
    @SerialName("houseflow/lightbulb")
    HouseflowLightbulb
}

@Serializable
data class Accessory(
    val name: String,
    @SerialName("room-name")
    val roomName: String,
    val type: AccessoryType,
)

enum class ServiceName(val codename: String) {
    TemperatureSensor("sensor-temperature"),
    HumiditySensor("sensor-humidity"),
}

enum class CharacteristicName(val codename: String) {
    CurrentTemperature("temperature-current"),
    CurrentHumidity("humidity-current"),
}

@Serializable
sealed class Characteristic() {
    abstract val name: CharacteristicName
}

@Serializable
data class CurrentTemperature(val temperature: Double): Characteristic() {
    override val name: CharacteristicName = CharacteristicName.CurrentTemperature
}

@Serializable
data class CurrentHumidity(val humidity: Double): Characteristic() {
    override val name: CharacteristicName = CharacteristicName.CurrentHumidity
}