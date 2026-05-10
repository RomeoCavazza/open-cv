/**
 * Shared audio and desktop notifications for document generation completion.
 */
export function requestNotificationPermission() {
    if ("Notification" in window && Notification.permission === "default") {
        Notification.requestPermission();
    }
}

let audioPrimed = false;

export function primeAudioPlayback() {
    if (audioPrimed) return;
    audioPrimed = true;
    try {
        const sound = new Audio('/assets/sounds/bell.mp3');
        sound.muted = true;
        const maybePromise = sound.play();
        if (maybePromise && typeof maybePromise.then === 'function') {
            maybePromise
                .then(() => {
                    sound.pause();
                    sound.currentTime = 0;
                    sound.muted = false;
                })
                .catch(() => {});
        }
    } catch (_) {}
}

export function showNotification(title, body) {
    if ("Notification" in window && Notification.permission === "granted") {
        new Notification(title, {
            body: body,
            icon: '/assets/img/logo.png'
        });
    }
}

export function playSuccessSound() {
    try {
        // Debounce: prevent overlapping sounds (within 1s)
        const lastPlayed = localStorage.getItem('last_success_sound_at');
        if (lastPlayed && Date.now() - parseInt(lastPlayed) < 1000) {
            return;
        }
        localStorage.setItem('last_success_sound_at', Date.now().toString());

        // 1. Audio Notification - Create fresh instance to avoid stale state
        const sound = new Audio('/assets/sounds/bell.mp3');
        sound.volume = 0.5;
        sound.play().catch(e => {
            console.warn("[Audio] Sound playback failed:", e);
            // Fallback: use a visual notification even if sound is blocked
            showNotification("RecruitAI", "Génération terminée avec succès !");
        });

        // 2. Desktop Notification (if backgrounded)
        if (document.hidden) {
            showNotification("RecruitAI", "Génération terminée avec succès !");
        }
    } catch (e) {
        console.error("[Audio] playSuccessSound error:", e);
    }
}
