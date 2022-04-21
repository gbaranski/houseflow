package com.gbaranski.houseflow

import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals

internal class HubFrameTest {
    @Test
    fun testDeserializeAccessoryConnected() {
        val json = """
        {
            "type": "accessory-connected",
            "accessory": {
                "id": "00000000-0000-0000-0000-000000000000",
                "name": "garage door",
                "room-name": "basement",
                "type": "mijia/hygro-thermometer"
            }
        }
        """
        val expected = HubFrame.AccessoryConnected(
            accessory = Accessory(
                id = "00000000-0000-0000-0000-000000000000",
                name = "garage door",
                roomName = "basement",
                type = AccessoryType.MijiaHygrothermometer
            )
        )
        println(Json.encodeToString(expected))

        val frame = Json.decodeFromString<HubFrame>(json)
        assertEquals(
            frame,
            expected
        )
    }
}
