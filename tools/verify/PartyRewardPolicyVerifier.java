import server.party.PartyRewardPolicy;

public final class PartyRewardPolicyVerifier {
    private PartyRewardPolicyVerifier() {
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    public static void main(String[] args) {
        require(PartyRewardPolicy.fullMesoShare(1200) == 1200,
                "every party member receives the full meso drop");
        require(PartyRewardPolicy.fullExperienceShare(875.0f) == 875.0f,
                "every party member receives the full party experience");
        require(PartyRewardPolicy.canShareExperience(30, 20, 40, 40),
                "eligible same-map member receives party experience");
        require(!PartyRewardPolicy.canShareExperience(19, 20, 40, 40),
                "underleveled member remains excluded");
        require(!PartyRewardPolicy.canShareExperience(1, 0, 41, 40),
                "existing anti-leech level range remains enforced");

        System.out.println("Party reward policy verification passed");
    }
}
