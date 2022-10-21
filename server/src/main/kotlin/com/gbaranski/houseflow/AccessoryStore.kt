package com.gbaranski.houseflow

class AccessoryStore {
    private val map = mutableMapOf<AccessoryID, Accessory>()

    fun connected(accessoryID: AccessoryID, accessory: Accessory) {
        map[accessoryID] = accessory
    }

    fun disconnected(accessoryID: AccessoryID) {
        map.remove(accessoryID)
    }
}