package com.gbaranski.houseflow

import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals

internal class ServerFrameTest {
    @Test
    fun testDeserializeReadCharacteristic() {
        val json = """
        {
            "id": 1,
            "type": "read-characteristic",
            "accessory-id": "00000000-0000-0000-0000-000000000000",
            "service": {
                "name": "temperature-sensor"
            },
            "characteristic": {
                "name": "current-temperature"
            }
        }
        """
        val expected = ServerFrame.ReadCharacteristic(
                id = 1,
                accessoryID = "00000000-0000-0000-0000-000000000000",
                service = TemperatureSensor,
                characteristic = CurrentTemperature(null),
            )

        val frame = Json.decodeFromString<ServerFrame>(json)
        assertEquals(
            frame,
            expected
        )
    }

    @Test
    fun testDeserializeWriteCharacteristic() {
        val json = """
        {
            "id": 1,
            "type": "write-characteristic",
            "accessory-id": "00000000-0000-0000-0000-000000000000",
            "service": {
                "name": "temperature-sensor"
            },
            "characteristic": {
                "name": "current-temperature",
                "temperature": 22.5
            }
        }
        """
        val expected = ServerFrame.WriteCharacteristic(
            id = 1,
            accessoryID = "00000000-0000-0000-0000-000000000000",
            service = TemperatureSensor,
            characteristic = CurrentTemperature(22.5),
        )

        val frame = Json.decodeFromString<ServerFrame>(json)
        assertEquals(
            frame,
            expected
        )
    }
}