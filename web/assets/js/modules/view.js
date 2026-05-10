import { 
    activeJobId, 
    activeTab, 
    setActiveJobId, 
    i18n 
} from '../state.js';
import * as router from '../router.js';
import * as iframeRender from '../render/iframe.js';

let pollInterval = null;

export async function updateIframe(options = {}) {
    if (!activeJobId) {
        iframeRender.resetIframeToEmptyState();
        return;
    }
    const offerSlug = activeJobId;

    const iframe = document.getElementById('iframe-doc');
    if (!iframe) return;

    // Point 3: Coordination with the Skeleton
    // Optimistically show skeleton if we know the backend is working on this tab
    const isGenerating = (() => {
        try {
            const raw = localStorage.getItem(`generating_target_${offerSlug}`);
            if (!raw) return false;
            const target = JSON.parse(raw);
            const targetKey = activeTab === 'cover' ? 'cover_letter' : activeTab;
            return target[targetKey] === true;
        } catch (_) { return false; }
    })();

    if (isGenerating) {
        console.log(`[View] ${offerSlug} is generating ${activeTab}, showing skeleton.`);
        iframeRender.renderIframeLoadingState(activeTab);
        return; // Prevent iframe navigation during generation to avoid flash/white screen
    }

    try {
        const res = await fetch(`/api/offres/${offerSlug}/instance`);
        let instanceSlug = offerSlug;
        
        if (res.ok) {
            const instance = await res.json();
            if (instance && instance.slug) {
                instanceSlug = instance.slug;
            }
        }
        
        if (pollInterval) {
            clearInterval(pollInterval);
            pollInterval = null;
        }

        const path = activeTab === 'restitution'
            ? '/restitution/index.html'
            : (activeTab === 'resume' ? '/resume/index.html' : '/cover-letter/index.html');

        const query = activeTab === 'restitution'
            ? `offer=${encodeURIComponent(offerSlug)}&instance=${encodeURIComponent(instanceSlug)}`
            : `id=${encodeURIComponent(instanceSlug)}&offer=${encodeURIComponent(offerSlug)}`;
        
        const newUrl = new URL(`${path}?${query}`, window.location.origin);

        // Force update with cache buster
        iframe.removeAttribute('srcdoc');
        const finalUrl = `${newUrl.href}${newUrl.search ? '&' : '?'}v=${Date.now()}`;
        console.log(`[View] Updating iframe to: ${finalUrl}`);
        iframe.src = finalUrl;
        router.updatePath();
    } catch (error) {
        console.warn("[View] Failed to update iframe", error);
    }
}
