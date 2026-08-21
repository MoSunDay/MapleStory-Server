import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicBoolean;
import net.server.coordinator.SessionSaveFence;

public final class SessionSaveFenceVerifier {
    private SessionSaveFenceVerifier() {
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static boolean establish(SessionSaveFence fence, int accountId, long sessionId) {
        SessionSaveFence.FencePermit permit = fence.tryEstablishSession(accountId, sessionId);
        if (permit == null) {
            return false;
        }
        permit.close();
        return true;
    }

    public static void main(String[] args) throws Exception {
        SessionSaveFence fence = new SessionSaveFence();

        require(establish(fence, 7, 100), "first session establishes generation");
        SessionSaveFence.FencePermit firstSave = fence.tryAcquireCurrentSession(7, 100);
        require(firstSave != null, "current session may save");
        firstSave.close();

        require(establish(fence, 7, 101), "new session advances generation");
        require(fence.tryAcquireCurrentSession(7, 100) == null, "old session is fenced from saving");
        require(fence.tryAcquireCurrentSession(7, 99) == null, "delayed older session stays fenced");
        require(!establish(fence, 7, 99), "older session cannot reclaim generation");

        SessionSaveFence.FencePermit secondSave = fence.tryAcquireCurrentSession(7, 101);
        require(secondSave != null, "new session may save");
        secondSave.close();

        final SessionSaveFence concurrentFence = new SessionSaveFence();
        require(establish(concurrentFence, 9, 200), "concurrent test session establishes");
        SessionSaveFence.FencePermit blockingSave = concurrentFence.tryAcquireCurrentSession(9, 200);
        require(blockingSave != null, "concurrent test save starts");
        final CountDownLatch establishStarted = new CountDownLatch(1);
        final AtomicBoolean establishFinished = new AtomicBoolean(false);
        Thread establishThread = new Thread(new Runnable() {
            @Override
            public void run() {
                establishStarted.countDown();
                establish(concurrentFence, 9, 201);
                establishFinished.set(true);
            }
        });
        establishThread.start();
        establishStarted.await();
        Thread.sleep(50);
        require(!establishFinished.get(), "new session waits for current DB save transaction");
        blockingSave.close();
        establishThread.join();
        require(establishFinished.get(), "new session proceeds after DB save transaction");
        require(concurrentFence.tryAcquireCurrentSession(9, 200) == null,
                "completed session switch fences the old saver");

        require(establish(fence, 8, 50), "accounts have independent generations");
        SessionSaveFence.FencePermit independentSave = fence.tryAcquireCurrentSession(8, 50);
        require(independentSave != null, "independent account may save");
        independentSave.close();

        System.out.println("Session save fence verification passed");
    }
}
