package src.block

import src.block.BlockProperty.NamedBlockProperty

class BlockState(block: Block, vararg properties: Byte) {
    private val block: Block
    val properties: ByteArray
    val model: BlockModel
    val hasModel: Boolean

    init {
        if (properties.size != block.attributes.size) throw RuntimeException()
        this.block = block
        this.properties = properties
        this.hasModel = block.hasModel(this)
        model = block.getModel(this)
    }

    override fun toString(): String {
        val str = StringBuilder(block.name)
        str.append(":[")
        for (j in properties.indices) {
            val i = properties[j].toInt()
            str.append(block.attributes[j].name)
            str.append('=')
            if (block.attributes[j].property is NamedBlockProperty) {
                str.append('\'')
                str.append((block.attributes[j].property as NamedBlockProperty).getName(i))
                str.append('\'')
            } else str.append(i)
            if (j < properties.size - 1) str.append(',')
        }
        str.append("]")
        return str.toString()
    }
}