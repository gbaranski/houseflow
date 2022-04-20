package com.gbaranski.houseflow

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator

@Serializable
enum class ServerFrameType {
    @SerialName("read-characteristic")
    ReadCharacteristic,
    @SerialName("write-characteristic")
    WriteCharacteristic,
}

@Serializable
data class ServerFrame(
    val type: ServerFrameType,
    @SerialName("accessory-id")
    val accessoryID: AccessoryID,
    val service: Service,
    val characteristic: Characteristic,
)