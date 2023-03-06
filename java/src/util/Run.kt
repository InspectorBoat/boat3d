package src.util;

/*@JvmInline
value class Run(private val array: ByteArray) {
    init {
        require(array.size == 8)
        array[7] = Byte.MAX_VALUE
    }

    var start: Byte
        get() = this.array[2]
        set(value: Byte) { this.array[2] = value }

    var end: Byte
        get() = this.array[3]
        set(value: Byte) { this.array[3] = value }

    var endVert: Byte
        get() = this.array[4]
        set(value: Byte) { this.array[4] = value }

    var endMat: Byte
        get() = this.array[5]
        set(value: Byte) { this.array[5] = value }


    var pointer: Byte
        get() = this.array[6]
        set(value: Byte) { this.array[6] = value }

    var row: Byte
        get() = this.array[7]
        set(value: Byte) { this.array[7] = value }

    val normal: Byte
        get() = this.array[0] and 0x000000ff

    val startX: Byte
        get() = (this.array[0] and -0x1000000) shr 24

    val endX: Byte
        get() = (this.array[4] and -0x1000000) shr 24

    val startY: Byte
        get() = (this.array[0] and 0x00ff0000) shr 16

    val endY: Byte
        get() = (this.array[4] and 0x00ff0000) shr 16

    val startZ: Byte
        get() = (this.array[0] and 0x0000ff00) shr 8

    val endZ: Byte
        get() = (this.array[4] and 0x0000ff00) shr 8

    val startWidth: Byte
        get() = (this.array[1] and -0x1000000) shr 24

    val endWidth: Byte
        get() = (this.array[5] and -0x1000000) shr 24

    val startHeight: Byte
        get() = (this.array[1] and 0x00ff0000) shr 16

    val endHeight: Byte
        get() = (this.array[5] and 0x00ff0000) shr 16

    val texture: Byte
        get() = this.array[1] and 0x0000ffff

}*/

//@JvmInline
/**
 * [start, end, minX, minY, maxX, maxY, zDepth, rowIndex, texture, pointer, pointer] * 16
 */
//value class Row(private val array: ByteArray) {
//    init {
//        require(array.size == 176)
//    }
//
//    fun start(column: Int):    Byte = this.array[column]
//    fun end(column: Int):      Byte = this.array[column + 1]
//    fun minX(column: Int):     Byte = this.array[column + 2]
//    fun minY(column: Int):     Byte = this.array[column + 3]
//    fun maxX(column: Int):     Byte = this.array[column + 4]
//    fun maxY(column: Int):     Byte = this.array[column + 5]
//    fun minZ(column: Int):     Byte = this.array[column + 6]
//    fun rowIndex(column: Int): Byte = this.array[column + 7]
//    fun texture(column: Int):  Byte = this.array[column + 8]
//    fun pointer(column: Int):  Int = ((this.array[column + 9].toInt() shl 8) and 0xff) or (this.array[column + 10].toInt() and 0xff)
//
//    fun start(column: Int, value: Byte)    { this.array[column]     = value }
//    fun end(column: Int, value: Byte)      { this.array[column + 1] = value }
//    fun minX(column: Int, value: Byte)     { this.array[column + 2] = value }
//    fun minY(column: Int, value: Byte)     { this.array[column + 3] = value }
//    fun maxX(column: Int, value: Byte)     { this.array[column + 4] = value }
//    fun maxY(column: Int, value: Byte)     { this.array[column + 5] = value }
//    fun minZ(column: Int, value: Byte)     { this.array[column + 6] = value }
//    fun rowIndex(column: Int, value: Byte) { this.array[column + 7] = value }
//    fun texture(column: Int, value: Byte)  { this.array[column + 8] = value }
//
//    fun start(column: Int, value: Int)    { this.array[column]     = value.toByte() }
//    fun end(column: Int, value: Int)      { this.array[column + 1] = value.toByte() }
//    fun minX(column: Int, value: Int)     { this.array[column + 2] = value.toByte() }
//    fun minY(column: Int, value: Int)     { this.array[column + 3] = value.toByte() }
//    fun maxX(column: Int, value: Int)     { this.array[column + 4] = value.toByte() }
//    fun maxY(column: Int, value: Int)     { this.array[column + 5] = value.toByte() }
//    fun minZ(column: Int, value: Int)     { this.array[column + 6] = value.toByte() }
//    fun rowIndex(column: Int, value: Int) { this.array[column + 7] = value.toByte() }
//    fun texture(column: Int, value: Int)  { this.array[column + 8] = value.toByte() }
//    fun pointer(column: Int, value: Int)  {
//        this.array[column + 9] = (value shl 8).toByte()
//        this.array[column + 10] = value.toByte()
//    }
//
//
//
//    fun endInc(column: Int)   { this.array[column + 1] ++ }
//}

class Run {
    var minZ: Int = 0
    var start: Int = 0
    var end: Int = 0

    var minX: Int = 0
    var minY: Int = 0

    var maxX: Int = 0
    var maxY: Int = 0

    var texture: Int = 0
    var row: Int = 0

    var pointer: Int = 0
}