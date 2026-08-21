package net.server.handlers;

import client.MapleClient;
import net.MaplePacketHandler;
import tools.data.input.SeekableLittleEndianAccessor;

/** Closes a client session after the standard v83 PLAYER_DC request. */
public final class PlayerDisconnectHandler implements MaplePacketHandler {
    @Override
    public void handlePacket(SeekableLittleEndianAccessor slea, MapleClient client) {
        client.disconnect(false, false);
    }

    @Override
    public boolean validateState(MapleClient client) {
        return true;
    }
}
