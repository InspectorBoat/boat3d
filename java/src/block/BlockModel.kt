package src.block

import src.world.Chunk.Norm

@JvmInline
value class BlockModel(val f: Array<BlockFace>) {
    operator fun get(norm: Norm): BlockFace {
        return this.f[norm.ordinal]
    }
    operator fun get(norm: Int): BlockFace {
        return this.f[norm]
    }
    companion object
}