import {
    activeJobId,
    activeTab,
    aiChatAttachments,
    setActiveJobId,
    setActiveTab,
    setSelectedLlmProvider,
    loadI18n,
    i18n
} from './state.js';
import * as ui from './ui.js';
import { safeClick } from './dom.js';
import * as router from './router.js';
import * as iframeRender from './render/iframe.js';
import { EVENTS, emit, on } from './modules/events.js';
import { updateIframe } from './modules/view.js';
import { ProfileController } from './controllers/ProfileController.js';
import { OfferController } from './controllers/OfferController.js';
import { IngestController } from './controllers/IngestController.js';

const profileController = new ProfileController();
const offerController = new OfferController();
const ingestController = new IngestController();

// --- Legacy Context ---
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

on(EVENTS.UPDATE_IFRAME, () => updateIframe());

on(EVENTS.LLM_PROVIDER_CHANGED, (data) => {
    document.querySelectorAll('.llm-pill[data-provider]').forEach(pill => {
        pill.classList.toggle('active', pill.dataset.provider === data.provider);
    });
});

on(EVENTS.NOTIFICATION, (data) => ui.showToast(data.message, data.type || 'info'));

on(EVENTS.GEN_STARTED, () => {
    const btn = document.getElementById('btn-ingest-run');
    if (btn) { btn.disabled = true; btn._oldText = btn.textContent; btn.textContent = '...'; }
});

on(EVENTS.GEN_COMPLETED, () => {
    const btn = document.getElementById('btn-ingest-run');
    if (btn) { btn.disabled = false; btn.textContent = btn._oldText || 'Generate Application'; }
    router.switchView('app');
    offerController.loadOffers();
    updateIframe();
});

on(EVENTS.GEN_FAILED, (data) => {
    const btn = document.getElementById('btn-ingest-run');
    if (btn) { btn.disabled = false; btn.textContent = btn._oldText || 'Generate Application'; }
    alert('Erreur: ' + (data.message || 'Inconnue'));
});

// --- Initialization ---

async function init() {
    console.log("[Dashboard] Initializing...");
    attachGlobalEventListeners();

    try {
        await loadI18n();
        await profileController.loadProfile();
        await offerController.loadOffers();
        
        router.initRouter({
            views: {
                ingest: document.getElementById('view-ingest'),
                app: document.getElementById('view-app'),
                profile: document.getElementById('view-profile')
            },
            callbacks: {
                onLoadOffers: () => offerController.loadOffers(),
                onResetIframe: () => iframeRender.resetIframeToEmptyState(),
                onLoadChatHistory: () => { if (typeof window.loadChatHistory === 'function') window.loadChatHistory(); }
            }
        });

        await router.handleRouting();
        ui.renderAiChatAttachments();
        console.log("[Dashboard] Ready.");
    } catch (e) {
        console.error("[Dashboard] Init Failed", e);
    }
}

function attachGlobalEventListeners() {
    // Navigation
    safeClick('nav-dashboard', (e) => { e.preventDefault(); router.switchView('ingest'); });
    safeClick('nav-app', (e) => { e.preventDefault(); router.switchView('app'); });
    safeClick('nav-profile', (e) => { e.preventDefault(); router.switchView('profile'); });

    // Controllers
    profileController.attachEventListeners();

    // Profile Add Buttons (Legacy/UI)
    safeClick('add-exp', () => document.getElementById('list-experiences').appendChild(ui.createExpRow()));
    safeClick('add-project', () => document.getElementById('list-projects').appendChild(ui.createExpRow()));
    safeClick('add-edu', () => document.getElementById('list-education').appendChild(ui.createEduRow()));
    safeClick('add-lang', () => document.getElementById('list-languages').appendChild(ui.createLangRow()));
    safeClick('add-skill-cat', () => document.getElementById('list-skills').appendChild(ui.createSkillRow()));
    safeClick('add-annexe', () => document.getElementById('prof-annexe-bulk-file').click());

    const annexeBulk = document.getElementById('prof-annexe-bulk-file');
    if (annexeBulk) annexeBulk.onchange = async (e) => {
        for (const file of Array.from(e.target.files)) {
            const data = await ui.readFileAsDataUrl(file);
            const row = ui.createAnnexeRow();
            row.dataset.fileData = data; row.dataset.fileName = file.name; row.dataset.fileType = file.type;
            row.querySelector('.annexe-name').value = file.name;
            document.getElementById('list-annexes').appendChild(row);
        }
        e.target.value = '';
    };

    // Tabs & View Actions
    document.querySelectorAll('.tab').forEach(btn => {
        btn.onclick = () => {
            setActiveTab(btn.dataset.target);
            document.querySelectorAll('.tab').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            updateIframe();
        };
    });

    safeClick('btn-reload-tab', () => updateIframe());
    safeClick('btn-download-pdf', () => {
        const iframe = document.getElementById('iframe-doc');
        if (iframe?.contentWindow) iframe.contentWindow.print();
    });

    // Selectors
    ui.setupSelector('llm-selector-ingest');
    ui.setupSelector('llm-selector-chat');
    ui.setupSelector('deliv-selector-ingest');

    // Ingest Action
    safeClick('btn-ingest-run', () => ingestController.runIngest());

    // Chat Attachments
    safeClick('ai-chat-attach-btn', () => document.getElementById('ai-chat-file-input').click());
    const chatFile = document.getElementById('ai-chat-file-input');
    if (chatFile) chatFile.onchange = async (e) => {
        for (const file of Array.from(e.target.files)) {
            const data = await ui.readFileAsDataUrl(file);
            aiChatAttachments.push({ name: file.name, content_type: file.type, data });
        }
        ui.renderAiChatAttachments();
        e.target.value = '';
    };
}

init();
