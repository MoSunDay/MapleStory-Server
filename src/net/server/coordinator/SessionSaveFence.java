/*
 This file is part of the HeavenMS Maple Story Server

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU Affero General Public License as published by
 the Free Software Foundation version 3 as published by the Free Software
 Foundation. You may not use, modify or distribute this program under any
 other version of the GNU Affero General Public License.
 */
package net.server.coordinator;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.locks.ReentrantLock;

/**
 * Fences whole-character saves by the newest session established for an
 * account. A granted permit holds the account stripe until the save finishes,
 * so a new login cannot race between the generation check and the DB commit.
 */
public final class SessionSaveFence {
    private static final int LOCK_STRIPES = 200;

    private final ConcurrentHashMap<Integer, Long> latestSessions = new ConcurrentHashMap<>();
    private final ReentrantLock[] accountLocks = new ReentrantLock[LOCK_STRIPES];

    public SessionSaveFence() {
        for (int i = 0; i < accountLocks.length; i++) {
            accountLocks[i] = new ReentrantLock();
        }
    }

    /**
     * Marks a session as the newest account generation and returns a permit
     * that keeps registration atomic with the caller's session-map update.
     * Session ids are process-wide monotonic values, so delayed older sessions
     * cannot reclaim ownership after a newer connection has been accepted.
     */
    public FencePermit tryEstablishSession(int accountId, long sessionId) {
        ReentrantLock lock = lockFor(accountId);
        lock.lock();

        Long latestSession = latestSessions.get(accountId);
        if (latestSession == null || sessionId > latestSession) {
            latestSessions.put(accountId, sessionId);
            return new FencePermit(lock);
        }
        if (sessionId == latestSession) {
            return new FencePermit(lock);
        }

        lock.unlock();
        return null;
    }

    /**
     * Returns a permit only for the newest session. The caller must close a
     * granted permit after its complete database operation has finished.
     */
    public FencePermit tryAcquireCurrentSession(int accountId, long sessionId) {
        ReentrantLock lock = lockFor(accountId);
        lock.lock();

        Long latestSession = latestSessions.get(accountId);
        if (latestSession != null && sessionId == latestSession) {
            return new FencePermit(lock);
        }

        lock.unlock();
        return null;
    }

    private ReentrantLock lockFor(int accountId) {
        return accountLocks[(accountId & Integer.MAX_VALUE) % accountLocks.length];
    }

    public static final class FencePermit implements AutoCloseable {
        private ReentrantLock lock;

        private FencePermit(ReentrantLock lock) {
            this.lock = lock;
        }

        @Override
        public void close() {
            if (lock != null) {
                lock.unlock();
                lock = null;
            }
        }
    }
}
