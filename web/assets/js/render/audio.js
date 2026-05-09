/**
 * Shared audio notification for document generation completion.
 */
const successSound = new Audio('/assets/sounds/bell.mp3');
successSound.volume = 0.5;

export function playSuccessSound() {
    successSound.currentTime = 0;
    successSound.play().catch(() => {});
}
