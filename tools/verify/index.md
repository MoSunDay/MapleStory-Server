# Verification Tools

- `Utf8StringCodecVerifier.java`: validates UTF-8 protocol length, strict decoding, fixed-width padding, and codepoint-safe legacy prefixes without starting the server.
- `CharacterNameVerifier.java`: validates Chinese name syntax, byte boundaries, blocked names, and database availability against the runtime configuration.
- `BuddyPacketVerifier.java`: validates that a 12-byte Chinese sender keeps the v83 buddy-request packet layout and never splits a UTF-8 code point.
