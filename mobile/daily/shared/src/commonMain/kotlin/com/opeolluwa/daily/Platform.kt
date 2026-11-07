package com.opeolluwa.daily

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform