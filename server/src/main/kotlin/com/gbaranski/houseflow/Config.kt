package com.gbaranski.houseflow

data class Secrets(
    val refresh_key: String,
    val access_key: String,
    val authorization_code_key: String,
)

// TODO: Make them generate at the first run
val secrets = Secrets(
    refresh_key = System.getenv("REFRESH_KEY"),
    access_key = System.getenv("ACCESS_KEY"),
    authorization_code_key = System.getenv("AUTHORIZATION_CODE_KEY"),
)