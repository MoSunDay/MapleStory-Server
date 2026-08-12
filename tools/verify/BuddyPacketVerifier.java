import java.util.Arrays;
import tools.MaplePacketCreator;
import tools.data.Utf8StringCodec;
import tools.data.input.ByteArrayByteStream;
import tools.data.input.GenericLittleEndianAccessor;

public final class BuddyPacketVerifier {
    private BuddyPacketVerifier() {
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    public static void main(String[] args) {
        String name = "中文测试";
        byte[] packet = MaplePacketCreator.requestBuddylistAdd(7, 9, name);
        GenericLittleEndianAccessor reader =
                new GenericLittleEndianAccessor(new ByteArrayByteStream(packet));

        reader.readShort();
        require(reader.readByte() == 9, "buddy request mode");
        require(reader.readInt() == 7, "sender id");
        require(name.equals(reader.readMapleAsciiString()), "complete UTF-8 sender name");
        require(reader.readInt() == 7, "repeated sender id");

        byte[] legacyName = reader.read(11);
        require(Arrays.equals(Arrays.copyOf(legacyName, 9), Utf8StringCodec.encode("中文测")),
                "legacy name uses a complete UTF-8 prefix");
        require(legacyName[9] == 0 && legacyName[10] == 0, "legacy name padding");
        require(packet.length == 61, "buddy packet layout remains fixed");

        System.out.println("Buddy packet verification passed");
    }
}
