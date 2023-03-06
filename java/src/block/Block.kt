@file:Suppress("UNCHECKED_CAST")

package src.block

abstract class Block protected constructor(name: String, vararg attributes: BlockAttribute) {
    val attributes: Array<BlockAttribute>
    val name: String
    val totalStates: Int

    init {
        this.name = name
        this.attributes = attributes as Array<BlockAttribute>
        var states = 1
        for (attribute in this.attributes) {
            states *= attribute.property.count + 1
        }
        totalStates = states
    }

    fun genStates(registry: Array<BlockState>, start: Int): Int {
        var end = start
        val states = Array(totalStates) { ByteArray(attributes.size) }
        var blockSize = 1
        for (prop in attributes.indices) {
            val size = attributes[prop].property.count + 1
            for (i in 0 until totalStates) {
                states[i][prop] = (i / blockSize % size).toByte()
            }
            blockSize *= size
        }
        var i = 0
        while (i < totalStates) {
            registry[end++] = BlockState(this, *states[i++])
        }
        return end
    }

    abstract fun getModel(blockState: BlockState): BlockModel
    abstract fun hasModel(blockState: BlockState): Boolean
}