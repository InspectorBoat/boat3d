package boat.util;

import boat.util.DebugHelper.log

@JvmInline
value class QuadRun(private val array: IntArray) {
    init {
        require(array.size == 8)
        array[7] = Int.MAX_VALUE
    }

    var startVec: Int
        get() = this.array[0]
        set(value: Int) { this.array[0] = value }

    var startTex: Int
        get() = this.array[1]
        set(value: Int) { this.array[1] = value }

    var startIndex: Int
        get() = this.array[2]
        set(value: Int) { this.array[2] = value }

    var endIndex: Int
        get() = this.array[3]
        set(value: Int) { this.array[3] = value }

    var endVec: Int
        get() = this.array[4]
        set(value: Int) { this.array[4] = value }

    var endTex: Int
        get() = this.array[5]
        set(value: Int) { this.array[5] = value }

    var bufferPos: Int
        get() = this.array[6]
        set(value: Int) { this.array[6] = value }

    var rowIndex: Int
        get() = this.array[7]
        set(value: Int) { this.array[7] = value }

    var startX: Int
        get() = (this.array[0] and -0x1000000) shr 24
        private set(value) {}

    var endX: Int
        get() = (this.array[4] and -0x1000000) shr 24
        private set(value) {}

    var startY: Int
        get() = (this.array[0] and 0x00ff0000) shr 16
        private set(value) {}

    var endY: Int
        get() = (this.array[4] and 0x00ff0000) shr 16
        private set(value) {}

    var startWidth: Int
        get() = (this.array[1] and -0x1000000) shr 24
        private set(value) {}

    var endWidth: Int
        get() = (this.array[5] and -0x1000000) shr 24
        private set(value) {}

    var startHeight: Int
        get() = (this.array[1] and 0x00ff0000) shr 16
        private set(value) {}

    var endHeight: Int
        get() = (this.array[5] and 0x00ff0000) shr 16
        private set(value) {}

    var texture: Int
        get() = this.array[1] and 0x0000ffff
        private set(value) {}

    fun reset() {
        this.array[0] = 0
        this.array[1] = 0
        this.array[2] = 0
        this.array[3] = 0
        this.array[4] = 0
        this.array[5] = 0
        this.array[6] = 0
        this.array[7] = 0
    }
}