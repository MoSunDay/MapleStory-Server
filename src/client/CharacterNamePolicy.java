package client;

import java.util.Locale;
import tools.data.Utf8StringCodec;

/** Pure validation rules for protocol-compatible character names. */
public final class CharacterNamePolicy {
    private static final String[] BLOCKED_NAMES = {
        "admin", "owner", "moderator", "intern", "donor", "administrator", "fredrick",
        "help", "helper", "alert", "notice", "maplestory", "fuck", "wizet", "fucking",
        "negro", "fuk", "fuc", "penis", "pussy", "asshole", "gay", "nigger", "homo",
        "suck", "cum", "shit", "shitty", "condom", "security", "official", "rape",
        "nigga", "sex", "tit", "boner", "orgy", "clit", "fatass", "bitch", "support",
        "gamemaster", "cock", "gaay", "gm", "operate", "master", "sysop", "party",
        "community", "message", "event", "test", "meso", "scania", "yata", "asiasoft",
        "henesys"
    };

    private CharacterNamePolicy() {
    }

    public static boolean isValid(String name) {
        if (name == null) {
            return false;
        }

        int encodedLength = Utf8StringCodec.encodedLength(name);
        if (encodedLength < 3 || encodedLength > 12) {
            return false;
        }

        for (int offset = 0; offset < name.length();) {
            int codePoint = name.codePointAt(offset);
            // The bundled Connector/J 5.1.6 cannot negotiate utf8mb4 for
            // supplementary-plane characters, so accept the BMP used by
            // Chinese names and reject values the database cannot round-trip.
            if (codePoint > Character.MAX_VALUE || !Character.isLetterOrDigit(codePoint)) {
                return false;
            }
            offset += Character.charCount(codePoint);
        }

        String lowerName = name.toLowerCase(Locale.ROOT);
        for (String blockedName : BLOCKED_NAMES) {
            if (lowerName.contains(blockedName)) {
                return false;
            }
        }
        return true;
    }
}
