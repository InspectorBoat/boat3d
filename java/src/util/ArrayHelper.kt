package src.util

object int {
    inline operator fun get(vararg values: Int): IntArray {
        return values
    }
}