import {
    activeJobId,
    activeTab,
    activeProfilId,
    aiChatAttachments,
    loadedProfileExtras,
    loadedProfileImage,
    selectedLlmProvider,
    collapsedOfferCategories,
    offerFlags,
    delivConfig,
    setActiveJobId,
    setActiveTab,
    setActiveProfilId,
    setLoadedProfileImage,
    setLoadedProfileExtras,
    setSelectedLlmProvider,
    loadI18n,
    saveOfferFlags,
    saveCollapsedCategories,
    setDelivConfig,
    i18n
} from './state.js';
import * as api from './api.js';
import * as ui from './ui.js';
import { clear, safeClick } from './dom.js';
import * as router from './router.js';
import * as iframeRender from './render/iframe.js';
import * as offerRender from './render/offers.js';
import { EVENTS, emit, on } from './modules/events.js';
import { ProfileController } from './controllers/ProfileController.js';
import { OfferController } from './controllers/OfferController.js';

const profileController = new ProfileController();
const offerController = new OfferController();

// --- Expose State & Utils for legacy scripts (chat.js) ---
window.state = {
    get activeTab() { return activeTab; },
    get activeJobId() { return activeJobId; },
    get aiChatAttachments() { return aiChatAttachments; },
    setSelectedLlmProvider,
    setActiveTab,
    setActiveJobId
};

// --- Event Subscriptions ---
on(EVENTS.OFFER_SELECTED, () => {
    offerController.loadOffers();
    updateIframe();
});

on(EVENTS.UPDATE_IFRAME, () => {
    updateIframe();
});

on(EVENTS.LLM_PROVIDER_CHANGED, (data) => {
    // Sync all LLM selectors in the DOM
    document.querySelectorAll('.llm-pill[data-provider]').forEach(pill => {
        if (pill.dataset.provider === data.provider) pill.classList.add('active');
        else pill.classList.remove('active');
    });
});

on(EVENTS.NOTIFICATION, (data) => {
    showToast(data.message, data.type || 'info');
});

// --- Toast System ---
function showToast(message, type = 'info') {
    let container = document.getElementById('toast-container');
    if (!container) {
        container = document.createElement('div');
        container.id = 'toast-container';
        document.body.appendChild(container);
    }

    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    container.appendChild(toast);

    setTimeout(() => {
        toast.classList.add('fade-out');
        setTimeout(() => toast.remove(), 300);
    }, 4000);
}

// --- Dashboard Logic ---

const views = {
    ingest: document.getElementById('view-ingest'),
    app: document.getElementById('view-app'),
    profile: document.getElementById('view-profile')
};

// Initialize router
router.initRouter({
    views,
    callbacks: {
        onLoadOffers: () => offerController.loadOffers(),
        onResetIframe: () => iframeRender.resetIframeToEmptyState(),
        onLoadChatHistory: () => {
            if (typeof window.loadChatHistory === 'function') window.loadChatHistory();
        }
    }
});

async function loadProfile() {
    return profileController.loadProfile();
}

// Temporarily expose for OfferController (Steps 3b/3c)
window.mutateOfferFlags = function(jobId, mutate) {
    const nextFlags = { ...(offerFlags[jobId] || {}) };
    mutate(nextFlags);
    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[jobId];
    else offerFlags[jobId] = nextFlags;
    saveOfferFlags();
    offerController.loadOffers();
};

window.selectOffer = function(jobId) {
    setActiveJobId(jobId);
    emit(EVENTS.OFFER_SELECTED, { jobId });
};

window.toggleOfferCategory = function(category) {
    const index = collapsedOfferCategories.indexOf(category);
    if (index >= 0) collapsedOfferCategories.splice(index, 1);
    else collapsedOfferCategories.push(category);
    saveCollapsedCategories();
    offerController.loadOffers();
};

async function updateIframe(options = {}) {
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
        console.warn("[Dashboard] Failed to update iframe", error);
    }
}

function renderAiChatAttachments() {
    const container = document.getElementById('ai-chat-attachments');
    if (!container) return;
    const t = i18n.translations[i18n.current];
    clear(container);

    if (!aiChatAttachments.length) {
        container.style.display = 'none';
        return;
    }

    container.style.display = 'flex';
    aiChatAttachments.forEach((file, index) => {
        const remove = document.createElement('button');
        remove.type = 'button';
        remove.className = 'ai-attachment-remove';
        remove.setAttribute('aria-label', t.attached_files);
        remove.innerText = '×';

        remove.onclick = () => {
            aiChatAttachments.splice(index, 1);
            renderAiChatAttachments();
        };

        const chip = document.createElement('div');
        chip.className = 'ai-attachment-chip';
        const nameSpan = document.createElement('span');
        nameSpan.className = 'ai-attachment-name';
        nameSpan.title = file.name;
        nameSpan.innerText = file.name;

        chip.appendChild(nameSpan);
        chip.appendChild(remove);
        container.appendChild(chip);
    });
}

// --- Init & Listeners ---

async function init() {
    console.log("[Dashboard] Initializing...");
    attachEventListeners();

    try {
        await loadI18n();
        await loadProfile();
        await offerController.loadOffers();
        await router.handleRouting();
        renderAiChatAttachments();
        console.log("[Dashboard] Initialization Complete.");
    } catch (e) {
        console.error("[Dashboard] Initialization Failed", e);
    }
}

function attachEventListeners() {
    const safeClick = (id, fn) => { const el = document.getElementById(id); if (el) el.onclick = fn; };

    safeClick('nav-dashboard', (e) => { e.preventDefault(); router.switchView('ingest'); });
    safeClick('nav-app', (e) => { e.preventDefault(); router.switchView('app'); });
    safeClick('nav-profile', (e) => { e.preventDefault(); router.switchView('profile'); });

    profileController.attachEventListeners();

    safeClick('add-exp', () => document.getElementById('list-experiences').appendChild(ui.createExpRow()));
    safeClick('add-project', () => document.getElementById('list-projects').appendChild(ui.createExpRow()));
    safeClick('add-edu', () => document.getElementById('list-education').appendChild(ui.createEduRow()));
    safeClick('add-lang', () => document.getElementById('list-languages').appendChild(ui.createLangRow()));
    safeClick('add-skill-cat', () => document.getElementById('list-skills').appendChild(ui.createSkillRow()));
    safeClick('add-annexe', () => document.getElementById('prof-annexe-bulk-file').click());

    const annexeBulk = document.getElementById('prof-annexe-bulk-file');
    if (annexeBulk) annexeBulk.onchange = async (e) => {
        const files = Array.from(e.target.files);
        for (const file of files) {
            const data = await ui.readFileAsDataUrl(file);
            const row = ui.createAnnexeRow();
            row.dataset.fileData = data;
            row.dataset.fileName = file.name;
            row.dataset.fileType = file.type;
            row.querySelector('.annexe-name').value = file.name;
            document.getElementById('list-annexes').appendChild(row);
        }
        e.target.value = '';
    };

    document.querySelectorAll('.tab').forEach(btn => {
        btn.onclick = () => {
            setActiveTab(btn.dataset.target);
            document.querySelectorAll('.tab').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            updateIframe({ syncChatHistory: false });
        };
    });

    safeClick('btn-reload-tab', () => updateIframe());
    safeClick('btn-download-pdf', () => {
        const iframe = document.getElementById('iframe-doc');
        if (iframe && iframe.contentWindow) iframe.contentWindow.print();
    });

    setupSelector('llm-selector-ingest');
    setupSelector('llm-selector-chat');
    setupSelector('deliv-selector-ingest');

    // Listeners for Generation UI
    on(EVENTS.GEN_STARTED, () => {
        const btn = document.getElementById('btn-ingest-run');
        if (btn) {
            btn.disabled = true;
            btn._oldText = btn.textContent;
            btn.textContent = '...';
        }
    });

    on(EVENTS.GEN_COMPLETED, () => {
        const btn = document.getElementById('btn-ingest-run');
        if (btn) {
            btn.disabled = false;
            btn.textContent = btn._oldText || 'Generate Application';
        }
        router.switchView('app');
        offerController.loadOffers();
        updateIframe();
    });

    on(EVENTS.GEN_FAILED, (data) => {
        const btn = document.getElementById('btn-ingest-run');
        if (btn) {
            btn.disabled = false;
            btn.textContent = btn._oldText || 'Generate Application';
        }
        alert('Erreur: ' + (data.message || 'Inconnue'));
    });

    safeClick('btn-ingest-run', async () => {
        const input = document.getElementById('job-input');
        if (!input || !input.value.trim()) return;

        emit(EVENTS.GEN_STARTED);

        try {
            const ingestRes = await api.ingestOffer(input.value);
            if (!ingestRes.job_id) throw new Error("Échec ingestion");

            emit(EVENTS.OFFER_INGESTED, { jobId: ingestRes.job_id });
            setActiveJobId(ingestRes.job_id);

            const delivs = document.getElementById('deliv-selector-ingest');
            const options = {
                restitution: delivs?.querySelector('[data-deliv="restitution"]')?.classList.contains('active') ?? true,
                resume: delivs?.querySelector('[data-deliv="resume"]')?.classList.contains('active') ?? true,
                cover_letter: delivs?.querySelector('[data-deliv="cover"]')?.classList.contains('active') ?? true,
            };

            await api.generateApplication(ingestRes.job_id, selectedLlmProvider, options);
            emit(EVENTS.GEN_COMPLETED, { jobId: ingestRes.job_id });
            emit(EVENTS.NOTIFICATION, { message: 'Application générée avec succès !', type: 'success' });

        } catch (e) {
            emit(EVENTS.GEN_FAILED, { message: e.message });
            emit(EVENTS.NOTIFICATION, { message: 'Erreur: ' + e.message, type: 'error' });
        }
    });

    safeClick('ai-chat-attach-btn', () => document.getElementById('ai-chat-file-input').click());
    const chatFile = document.getElementById('ai-chat-file-input');
    if (chatFile) chatFile.onchange = async (e) => {
        const files = Array.from(e.target.files);
        for (const file of files) {
            const data = await ui.readFileAsDataUrl(file);
            aiChatAttachments.push({ name: file.name, content_type: file.type, data });
        }
        renderAiChatAttachments();
        e.target.value = '';
    };
}

function setupSelector(containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    container.querySelectorAll('.llm-pill').forEach(pill => {
        const prov = pill.dataset.provider;
        const deliv = pill.dataset.deliv;

        if (prov) {
            if (selectedLlmProvider === prov) pill.classList.add('active');
            else pill.classList.remove('active');
        } else if (deliv) {
            const val = delivConfig[deliv];
            if (val === true) pill.classList.add('active');
            else if (val === false) pill.classList.remove('active');
        }

        pill.onclick = (e) => {
            e.preventDefault();
            if (prov) {
                setSelectedLlmProvider(prov);
                emit(EVENTS.LLM_PROVIDER_CHANGED, { provider: prov });
            } else if (deliv) {
                const newVal = !delivConfig[deliv];
                delivConfig[deliv] = newVal;
                setDelivConfig({ ...delivConfig });
                if (newVal) pill.classList.add('active');
                else pill.classList.remove('active');
            }
        };
    });
}

init();
