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
        const currentUrl = new URL(iframe.src, window.location.origin);
        currentUrl.searchParams.delete('v'); // Strip version for comparison
        
        // Only update if the base path or query has changed
        if (currentUrl.pathname !== newUrl.pathname || currentUrl.search !== newUrl.search) {
            iframe.removeAttribute('srcdoc');
            iframe.src = `${newUrl.href}&v=${Date.now()}`;
            router.updatePath();
        }
    } catch (error) {
        console.warn("[View] Failed to update iframe", error);
    }
}
