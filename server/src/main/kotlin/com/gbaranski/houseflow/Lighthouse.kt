package com.gbaranski.houseflow

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator

typealias FrameID = Int

@Serializable
sealed class ServerFrame(val type: String) {
    @Serializable
    @SerialName("read-characteristic")
    data class ReadCharacteristic(
        val id: FrameID,
        @SerialName("accessory-id")
        val accessoryID: AccessoryID,
        val service: Service,
        val characteristic: Characteristic,
    ) : ServerFrame("read-characteristic")

    @Serializable
    @SerialName("write-characteristic")
    data class WriteCharacteristic(
        val id: FrameID,
        @SerialName("accessory-id")
        val accessoryID: AccessoryID,
        val service: Service,
        val characteristic: Characteristic,
    ) : ServerFrame("write-characteristic")
}

@Serializable
enum class ResultStatus {
    success,
    error
}

@Serializable
data class Result<T>(
    val status: ResultStatus,
    val body: T?,
)

@Serializable
sealed class HubFrame(val type: String) {
    @Serializable
    @SerialName("accessory-connected")
    data class AccessoryConnected(val accessory: Accessory) : HubFrame("accessory-connected")

    @Serializable
    @SerialName("accessory-disconnected")
    data class AccessoryDisconnected(@SerialName("accessory-id") val accessoryID: AccessoryID) :
        HubFrame("accessory-disconnected")

    @Serializable
    @SerialName("update-characteristic")
    data class UpdateCharacteristic(
        @SerialName("accessory-id")
        val accessoryID: AccessoryID,
        val service: Service,
        val characteristic: Characteristic
    ) : HubFrame("update-characteristic")

    @Serializable
    @SerialName("read-characteristic-result")
    data class ReadCharacteristicResult(
        val id: FrameID,
        val result: Result<Characteristic>
    ) : HubFrame("read-characteristic-result")

    @Serializable
    @SerialName("write-characteristic-result")
    data class WriteCharacteristicResult(
        val id: FrameID,
        val result: Result<Characteristic> // TODO: Change to some empty type
    ) : HubFrame("write-characteristic-result")
}
