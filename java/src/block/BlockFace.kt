package src.block

/*@JvmInline
value class BlockFace(private val array: ByteArray) {
    init {
        require(array.size == 8)
    }
    val normal: Byte
        get() = this.array[0]

    val z: Byte
        get() = this.array[1]
    val y: Byte
        get() = this.array[2]
    val x: Byte
        get() = this.array[3]

    val texture: Byte
        get() = this.array[4]
    val height: Byte
        get() = this.array[6]
    val width: Byte
        get() = this.array[7]

    companion object {
        fun new(normal: Norm, x: Byte, y: Byte, z: Byte, w: Byte, h: Byte, t: Byte): BlockFace {
            assert(
                    x     in 0..15 &&
                    y     in 0..15 &&
                    z     in 0..15 &&
                    w     in 0..15 &&
                    h     in 0..15 &&
                    x + w in 0..15 &&
                    y + h in 0..15
            )
            return BlockFace(byteArrayOf(normal.ordinal.toByte(), z, y, x, t, t, h, w))
        }
    }
}*/

class BlockFace(@JvmField val n: Int,
                @JvmField val z: Int,
                @JvmField val y: Int,
                @JvmField val x: Int,
                @JvmField val t: Int,
                @JvmField val h: Int,
                @JvmField val w: Int) {
    @JvmField
    val vertex: Int = (x shl 24) or (y shl 16) or (z shl 8) or n
    @JvmField
    val material: Int = (w shl 24) or (h shl 16) or (t)
    init {
        require(x in 0..15)
        require(y in 0..15)
        require(z in 0..15)
        require(n in 0..5)
        require(w in 0..15)
        require(h in 0..15)
    }
}