package src.block

import src.world.Chunk.Norm.*

class GrassBlock : Block("grass", BlockAttribute("snowy", BlockProperty.BOOLEAN)) {
    override fun getModel(blockState: BlockState): BlockModel {
        return if (blockState.properties[0].toInt() == 0) BlockModel(arrayOf(
                BlockFace(SOUTH.ordinal,
                        x =  0, y =  0, z = 0,
                        w = 15, h = 15, t = 0),
                BlockFace(NORTH.ordinal,
                        x =  0, y =  0, z = 0,
                        w = 15, h = 15, t = 0),
        )) else BlockModel(arrayOf(
                BlockFace(SOUTH.ordinal,
                        x =  0, y =  0, z = 0,
                        w = 15, h = 15, t = 0),
                BlockFace(NORTH.ordinal,
                        x =  0, y =  0, z = 0,
                        w = 15, h = 15, t = 0),
                ))
    }

    override fun hasModel(blockState: BlockState): Boolean {
        return true
    }
}