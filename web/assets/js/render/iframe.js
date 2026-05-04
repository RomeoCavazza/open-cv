/**
 * Gère l'état de l'iframe d'aperçu.
 */

export function resetIframeToEmptyState() {
    const iframe = document.getElementById('iframe-doc');
    if (!iframe) return;

    iframe.removeAttribute('srcdoc');
    iframe.src = '/assets/templates/iframe-empty.html';
    
    window.activeInstanceSlug = null;
    window.activeInstanceData = null;
    window.activeResolvedOfferSlug = null;
}

