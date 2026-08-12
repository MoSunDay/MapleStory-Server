import java.util.Arrays;
import tools.data.Utf8StringCodec;
import tools.data.input.ByteArrayByteStream;
import tools.data.input.GenericLittleEndianAccessor;
import tools.data.output.MaplePacketLittleEndianWriter;

public final class Utf8StringCodecVerifier {
    private Utf8StringCodecVerifier() {
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static void expectInvalid(Runnable operation, String message) {
        try {
            operation.run();
        } catch (IllegalArgumentException expected) {
            return;
        }
        throw new AssertionError(message);
    }

    public static void main(String[] args) {
        require(Utf8StringCodec.encodedLength("test1") == 5, "ASCII byte length");
        require(Utf8StringCodec.encodedLength("测试一") == 9, "Chinese byte length");
        require("测试一".equals(Utf8StringCodec.decode(Utf8StringCodec.encode("测试一"))),
                "Chinese round trip");

        byte[] fixed = Utf8StringCodec.encodeFixed("测试一", 13);
        require(fixed.length == 13, "fixed field width");
        require(Arrays.equals(Arrays.copyOf(fixed, 9), Utf8StringCodec.encode("测试一")),
                "fixed field content");
        require(fixed[9] == 0 && fixed[12] == 0, "fixed field padding");

        byte[] legacyPrefix = Utf8StringCodec.encodeFixedPrefix("中文测试", 11);
        require(legacyPrefix.length == 11, "legacy prefix preserves fixed width");
        require(Arrays.equals(Arrays.copyOf(legacyPrefix, 9), Utf8StringCodec.encode("中文测")),
                "legacy prefix stops at a UTF-8 code point boundary");
        require(legacyPrefix[9] == 0 && legacyPrefix[10] == 0,
                "legacy prefix pads the unused bytes");

        MaplePacketLittleEndianWriter writer = new MaplePacketLittleEndianWriter();
        writer.writeMapleAsciiString("测试一");
        byte[] packet = writer.getPacket();
        require(packet.length == 11 && packet[0] == 9 && packet[1] == 0,
                "Maple prefix uses UTF-8 byte length");
        GenericLittleEndianAccessor reader =
                new GenericLittleEndianAccessor(new ByteArrayByteStream(packet));
        require("测试一".equals(reader.readMapleAsciiString()), "Maple UTF-8 round trip");

        writer = new MaplePacketLittleEndianWriter();
        writer.writeFixedString("测试一", 13);
        require(writer.getPacket().length == 13, "writer preserves fixed protocol width");

        expectInvalid(new Runnable() {
            @Override
            public void run() {
                Utf8StringCodec.encodeFixed("中文测试甲", 12);
            }
        }, "over-width fixed field must fail");
        expectInvalid(new Runnable() {
            @Override
            public void run() {
                Utf8StringCodec.decode(new byte[] {(byte) 0xE4, (byte) 0xB8});
            }
        }, "truncated UTF-8 must fail");

        System.out.println("UTF-8 codec verification passed");
    }
}
