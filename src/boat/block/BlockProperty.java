package boat.block;

public class BlockProperty {
    public final int count;

    public BlockProperty(int max) {
        this.count = max;
    }

    public static class NamedBlockProperty extends BlockProperty {
        public final String[] names;

        public NamedBlockProperty(String... names) {
            super(names.length - 1);
            this.names = new String[names.length];
            System.arraycopy(names, 0, this.names, 0, names.length);
        }

        public String getName(int id) {
            return this.names[id];
        }
    }

    public static final BlockProperty POWER = new BlockProperty(15);
    public static final NamedBlockProperty COLOR = new NamedBlockProperty("white", "orange", "magenta", "light_blue", "yellow", "lime", "pink", "gray", "light_gray", "cyan", "purple", "blue", "brown", "green", "red", "black");
    public static final NamedBlockProperty WIRE_CONNECTION = new NamedBlockProperty("none", "up", "side");
    public static final NamedBlockProperty HORIZONTAL_FACING = new NamedBlockProperty("north", "east", "south", "west");
    public static final NamedBlockProperty VERTICAL_FACING = new NamedBlockProperty("up", "down");
    public static final NamedBlockProperty HORIZONTAL_AXIS = new NamedBlockProperty("x", "y");
    public static final NamedBlockProperty AXIS = new NamedBlockProperty("x", "y", "z");
    public static final NamedBlockProperty WOOD_SPECIES = new NamedBlockProperty("oak", "spruce", "birch", "jungle", "acacia", "dark_oak", "mangrove", "crimson", "warped");
    public static final NamedBlockProperty BOOLEAN = new NamedBlockProperty("true", "false");
}
