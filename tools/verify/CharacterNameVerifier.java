import client.CharacterNamePolicy;

public final class CharacterNameVerifier {
    private CharacterNameVerifier() {
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    public static void main(String[] args) {
        require(CharacterNamePolicy.isValid("测试一"), "available Chinese name syntax");
        require(CharacterNamePolicy.isValid("中文测试"), "maximum 12-byte Chinese name");
        require(!CharacterNamePolicy.isValid("ab"), "minimum UTF-8 byte length");
        require(!CharacterNamePolicy.isValid("测试一甲乙"), "maximum UTF-8 byte length");
        require(!CharacterNamePolicy.isValid("测试!"), "letters and digits only");
        require(!CharacterNamePolicy.isValid("𠀀名"), "supplementary plane is not database-compatible");
        require(!CharacterNamePolicy.isValid("test"), "blocked name");
        System.out.println("Character name verification passed");
    }
}
