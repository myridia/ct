package com.zetcode
//import java.security.SecureRandom;
import kotlin.random.Random

class Simple {

    private val name = "Simple"
    fun info() = "This is $name class"
}

fun main() {

    val s = Simple()
    println(s)
    println(s.info())
}
