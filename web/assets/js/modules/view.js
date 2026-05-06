import { 
    activeJobId, 
    activeTab, 
    setActiveJobId, 
    i18n 
} from '../state.js';
import * as router from '../router.js';
import * as iframeRender from '../render/iframe.js';

export async function updateIframe(options = {}) {
    if (!activeJobId) {
        iframeRender.resetIframeToEmptyState();
        return;
    }
    const initialTab = activeTab;
    const offerSlug = activeJobId;

    const iframe = document.getElementById('iframe-doc');
    if (!iframe) return;

    const path = activeTab === 'restitution'
        ? '/restitution/index.html'
        : (activeTab === 'resume' ? '/resume/index.html' : '/cover-letter/index.html');

    if (!offerSlug || offerSlug === 'null') return;

    try {
        const res = await fetch(`/api/offres/${offerSlug}/instance`);
        let instanceSlug = offerSlug;
        if (res.ok) {
            const instance = await res.json();
            if (instance && instance.slug) {
                instanceSlug = instance.slug;
            }
        }

        if (activeJobId !== offerSlug || activeTab !== initialTab) return;

        const query = activeTab === 'restitution'
            ? `offer=${encodeURIComponent(offerSlug)}&instance=${encodeURIComponent(instanceSlug)}`
            : `id=${encodeURIComponent(instanceSlug)}&offer=${encodeURIComponent(offerSlug)}`;

        iframe.removeAttribute('srcdoc');
        iframe.src = `${path}?${query}&v=${Date.now()}`;
        router.updatePath();
    } catch (error) {
        console.warn("[View] Failed to update iframe", error);
    }
}
