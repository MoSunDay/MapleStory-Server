package tools.data;

import java.nio.ByteBuffer;
import java.nio.CharBuffer;
import java.nio.charset.CharacterCodingException;
import java.nio.charset.Charset;
import java.nio.charset.CharsetDecoder;
import java.nio.charset.CharsetEncoder;
import java.nio.charset.CodingErrorAction;
import java.util.Arrays;

/**
 * Strict UTF-8 conversion helpers for Maple protocol strings.
 *
 * Protocol lengths are byte counts, so conversion must happen before a
 * length is written or a fixed-width field is padded.
 */
public final class Utf8StringCodec {
    private static final Charset UTF8 = Charset.forName("UTF-8");

    private Utf8StringCodec() {
    }

    public static byte[] encode(String value) {
        if (value == null) {
            throw new IllegalArgumentException("Protocol string must not be null");
        }

        CharsetEncoder encoder = UTF8.newEncoder()
                .onMalformedInput(CodingErrorAction.REPORT)
                .onUnmappableCharacter(CodingErrorAction.REPORT);
        try {
            ByteBuffer encoded = encoder.encode(CharBuffer.wrap(value));
            byte[] bytes = new byte[encoded.remaining()];
            encoded.get(bytes);
            return bytes;
        } catch (CharacterCodingException ex) {
            throw new IllegalArgumentException("Protocol string is not valid Unicode", ex);
        }
    }

    public static String decode(byte[] bytes) {
        if (bytes == null) {
            throw new IllegalArgumentException("Protocol bytes must not be null");
        }

        CharsetDecoder decoder = UTF8.newDecoder()
                .onMalformedInput(CodingErrorAction.REPORT)
                .onUnmappableCharacter(CodingErrorAction.REPORT);
        try {
            return decoder.decode(ByteBuffer.wrap(bytes)).toString();
        } catch (CharacterCodingException ex) {
            throw new IllegalArgumentException("Protocol string contains invalid UTF-8", ex);
        }
    }

    public static int encodedLength(String value) {
        return encode(value).length;
    }

    public static byte[] encodeFixed(String value, int byteLength) {
        if (byteLength < 0) {
            throw new IllegalArgumentException("Fixed string length must not be negative");
        }

        byte[] encoded = encode(value);
        if (encoded.length > byteLength) {
            throw new IllegalArgumentException(
                    "UTF-8 string needs " + encoded.length + " bytes, field allows " + byteLength);
        }
        return Arrays.copyOf(encoded, byteLength);
    }

    /**
     * Encodes the longest complete UTF-8 prefix that fits a legacy fixed field.
     * Use this only when the protocol also carries the complete value elsewhere.
     */
    public static byte[] encodeFixedPrefix(String value, int byteLength) {
        if (byteLength < 0) {
            throw new IllegalArgumentException("Fixed string length must not be negative");
        }

        byte[] encoded = encode(value);
        if (encoded.length <= byteLength) {
            return Arrays.copyOf(encoded, byteLength);
        }

        int prefixLength = 0;
        for (int offset = 0; offset < value.length();) {
            int codePoint = value.codePointAt(offset);
            int codePointLength = encodedLength(new String(Character.toChars(codePoint)));
            if (prefixLength + codePointLength > byteLength) {
                break;
            }
            prefixLength += codePointLength;
            offset += Character.charCount(codePoint);
        }
        byte[] result = new byte[byteLength];
        System.arraycopy(encoded, 0, result, 0, prefixLength);
        return result;
    }
}
