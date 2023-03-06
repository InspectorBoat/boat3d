package src.block

import src.block.Block
import src.block.BlockModel
import src.block.BlockState

class AirBlock : Block("air") {
    override fun getModel(blockState: BlockState): BlockModel {
        return BlockModel(arrayOf())
    }

    override fun hasModel(blockState: BlockState): Boolean {
        return false
    }
}