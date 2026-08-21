package server.party;

public final class PartyRewardPolicy {
    private PartyRewardPolicy() {
    }

    public static int fullMesoShare(int droppedMeso) {
        return droppedMeso;
    }

    public static float fullExperienceShare(float partyExperience) {
        return partyExperience;
    }

    public static boolean canShareExperience(int memberLevel, int minimumLevel,
                                             int killerLevel, int maximumLevelRange) {
        return memberLevel >= minimumLevel
                && Math.abs(killerLevel - memberLevel) < maximumLevelRange;
    }
}
