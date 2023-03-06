//package boat.block;
//
//public abstract class SlabBlock extends Block {
//    protected SlabBlock() {
//        super("slab", new BlockAttribute("top", BlockProperty.BOOLEAN));
//    }
//
//    @Override
//    protected boolean isFullCube(BlockState blockState) {
//        return false;
//    }
//
//    @Override
//    public BlockModel getModel(BlockState blockState) {
//        if (blockState.properties[0] == 0) return new BlockModel(new int[][] {
//                {0x00000000, 0x0f070f00},
//                {0x00000f01, 0x0f070f00},
//                {0x00000002, 0x0f070f00},
//                {0x0f000003, 0x0f070f00},
//                {0x00000004, 0x0f0f0f00},
//                {0x00070005, 0x0f0f0f00}
//        });
//        return new BlockModel(new int[][] {
//                {0x00080000, 0x0f070f02},
//                {0x00080f01, 0x0f070f02},
//                {0x00080002, 0x0f070f02},
//                {0x0f080003, 0x0f070f02},
//                {0x00080004, 0x0f0f0f02},
//                {0x000f0005, 0x0f0f0f02},
//        });
//    }
//}
