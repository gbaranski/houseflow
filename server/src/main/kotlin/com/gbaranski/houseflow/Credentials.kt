package com.gbaranski.houseflow


data class Credentials(
    val accessKeySecret: String,
    val refreshKeySecret: String,
    val authorizationCodeSecret: String,
    val googleClientID: String,
    val googleClientSecret: String,
)

fun credentialsFromEnv() = Credentials(
    googleClientID = System.getenv("GOOGLE_CLIENT_ID"),
    googleClientSecret = System.getenv("GOOGLE_CLIENT_SECRET")
)