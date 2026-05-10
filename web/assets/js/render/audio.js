/**
 * Shared audio and desktop notifications for document generation completion.
 */
export function requestNotificationPermission() {
    if ("Notification" in window && Notification.permission === "default") {
        Notification.requestPermission();
    }
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
        // Debounce: prevent multiple tabs from playing sound at once (within 5s)
        const lastPlayed = localStorage.getItem('last_success_sound_at');
        if (lastPlayed && Date.now() - parseInt(lastPlayed) < 5000) {
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
