import * as state from './state.js';
import * as api from './api.js';
import * as ui from './ui.js';
import { clear } from './dom.js';
import * as router from './router.js';
import * as iframeRender from './render/iframe.js';
import * as offerRender from './render/offers.js';
import { EVENTS, emit } from './modules/events.js';

// --- Expose State & Utils for legacy scripts (chat.js) ---
window.state = {
    get selectedLlmProvider() { return state.selectedLlmProvider; },
    get activeTab() { return state.activeTab; },
    setSelectedLlmProvider: state.setSelectedLlmProvider,
    setActiveTab: state.setActiveTab
};

// --- Event Subscriptions ---
window.AppEvents.on(EVENTS.OFFER_SELECTED, () => {
    loadOffers();
    updateIframe();
});
window.updateIframe = updateIframe;

// --- Dashboard Logic ---

let offersLoadSeq = 0;

const views = {
    ingest: document.getElementById('view-ingest'),
    app: document.getElementById('view-app'),
    profile: document.getElementById('view-profile')
};

// Initialize router
router.initRouter({
    views,
    callbacks: {
        onLoadOffers: () => loadOffers(),
        onResetIframe: () => iframeRender.resetIframeToEmptyState(),
        onLoadChatHistory: () => {
            if (typeof window.loadChatHistory === 'function') window.loadChatHistory();
        }
    }
});

async function loadProfile() {
    try {
        const profileResponse = await api.fetchProfile();
        const content = profileResponse.content;
        state.setActiveProfilId(profileResponse.id || null);

        state.setLoadedProfileExtras(Object.fromEntries(
            Object.entries(content).filter(([key]) => ![
                'profile', 'apprenticeship', 'experiences', 'projects',
                'education', 'languages', 'skills', 'labels'
            ].includes(key))
        ));

        const setVal = (id, val) => { const el = document.getElementById(id); if (el) el.value = val || ""; };
        setVal('prof-firstname', content.profile?.firstname);
        setVal('prof-lastname', content.profile?.lastname);
        setVal('prof-title', content.profile?.title);
        setVal('prof-offer-type', content.profile?.offer_type || "Alternance");
        setVal('prof-duration', content.apprenticeship?.duration);
        setVal('prof-rhythm', content.apprenticeship?.rhythm);
        setVal('prof-pitch', content.profile?.pitch);
        setVal('prof-location', content.profile?.location);
        setVal('prof-phone', content.profile?.phone);
        setVal('prof-email', content.profile?.email);
        setVal('prof-linkedin', content.profile?.linkedin);
        setVal('prof-website', content.profile?.website);
        setVal('prof-github', content.profile?.github);

        setVal('prof-resume-template', ui.stringifyDocument(content.documents?.resume_template || content.resume_template));
        setVal('prof-cover-letter-template', ui.stringifyDocument(content.documents?.cover_letter_template || content.cover_letter_template));

        state.setLoadedProfileImage(content.profile?.image || "");
        setVal('prof-image-base64', (content.profile?.image === "persisted:bytea") ? "" : state.loadedProfileImage);

        const preview = document.getElementById('prof-image-preview');
        const placeholder = document.getElementById('preview-placeholder');
        if (preview && content.profile?.image) {
            const imageUrl = (content.profile.image === "persisted:bytea")
                ? `/api/profile/active/photo?t=${Date.now()}`
                : content.profile.image;
            preview.style.backgroundImage = `url(${imageUrl})`;
            if (placeholder) placeholder.style.display = 'none';
        } else if (preview) {
            preview.style.backgroundImage = '';
            if (placeholder) placeholder.style.display = 'block';
        }

        ui.renderList('list-experiences', content.experiences || [], ui.createExpRow);
        ui.renderList('list-projects', content.projects || [], ui.createExpRow);
        ui.renderList('list-education', content.education || [], ui.createEduRow);
        ui.renderList('list-languages', content.languages || [], ui.createLangRow);
        ui.renderList('list-skills', content.skills || [], ui.createSkillRow);

        try {
            const annexes = await api.fetchAnnexes();
            ui.renderList('list-annexes', annexes || [], ui.createAnnexeRow);
        } catch (annexError) {
            console.error("Échec du chargement des annexes", annexError);
        }

        ui.updateUIStrings();
    } catch (e) { console.warn("Profile load failed", e); }
}

async function loadOffers() {
    const loadSeq = ++offersLoadSeq;
    try {
        const data = await api.fetchOffers();
        const offers = data.entries || [];
        if (loadSeq !== offersLoadSeq) return;

        renderDashboardApplications(offers);
        renderDashboardTreatedOffers(offers);
        renderOldOffers(offers);

        const list = document.getElementById('offers-list');
        if (!list) return;
        clear(list);
        const inboxLabel = state.i18n.translations[state.i18n.current].inbox;

        const visibleInboxOffers = offers.filter((offer) => {
            const flags = state.offerFlags[offer.job_id] || {};
            return !flags.archived && !flags.oldCv && !flags.deleted;
        });

        const visibleArchivedOffers = offers.filter((offer) => {
            const flags = state.offerFlags[offer.job_id] || {};
            return flags.archived && !flags.oldCv && !flags.deleted;
        });

        const inboxCount = visibleInboxOffers.length || offers.length;
        const inboxHeader = document.getElementById('sidebar-inbox-header');
        if (inboxHeader) {
            inboxHeader.className = 'sidebar-section-header';
            inboxHeader.innerHTML = '';

            const titleSpan = document.createElement('span');
            titleSpan.className = 'sidebar-section-title';
            titleSpan.textContent = inboxLabel;

            const badge = document.createElement('span');
            badge.className = 'sidebar-section-badge';
            badge.textContent = inboxCount;

            inboxHeader.appendChild(titleSpan);
            inboxHeader.appendChild(badge);
        }

        const groups = {};
        visibleInboxOffers.forEach(o => {
            const cat = o.category || state.i18n.translations[state.i18n.current].others;
            if (!groups[cat]) groups[cat] = [];
            groups[cat].push(o);
        });

        Object.keys(groups).sort().forEach(cat => {
            const isDefault = cat === state.i18n.translations[state.i18n.current].others || cat.toUpperCase().includes("INBOX");
            const isCollapsed = state.collapsedOfferCategories.includes(cat);
            let groupContainer = list;

            if (!isDefault) {
                const catDiv = document.createElement('div');
                catDiv.className = `sidebar-section-header offer-group-header${isCollapsed ? ' collapsed' : ''}`;

                const toggle = document.createElement('div');
                toggle.className = 'offer-group-toggle';

                const label = document.createElement('span');
                const translatedCat = state.i18n.translations[state.i18n.current][cat.toLowerCase()] || cat;
                label.className = 'sidebar-section-title';
                label.textContent = translatedCat;

                const count = document.createElement('span');
                count.className = 'sidebar-section-badge';
                count.textContent = groups[cat].length;

                toggle.appendChild(label);
                toggle.appendChild(count);
                catDiv.appendChild(toggle);

                catDiv.onclick = () => toggleOfferCategory(cat);
                list.appendChild(catDiv);
                groupContainer = document.createElement('div');
                groupContainer.style.display = isCollapsed ? 'none' : 'block';
                list.appendChild(groupContainer);
            }

            groups[cat].forEach(o => {
                const isActive = state.activeJobId === o.job_id;
                const flags = state.offerFlags[o.job_id] || {};
                const isLocked = !!flags.locked;
                const isArchived = !!flags.archived;
                const hasFlag = isLocked || !!flags.archived;
                const card = offerRender.createOfferCard(o, {
                    isActive,
                    isLocked,
                    isArchived,
                    hasFlag,
                    archivedView: false,
                });

                card.querySelector('[data-action="lock"]').onclick = (event) => {
                    event.stopPropagation();
                    mutateOfferFlags(o.job_id, (nextFlags) => {
                        nextFlags.locked = !nextFlags.locked;
                    });
                };

                card.querySelector('[data-action="archive"]').onclick = (event) => {
                    event.stopPropagation();
                    mutateOfferFlags(o.job_id, (nextFlags) => {
                        nextFlags.archived = !nextFlags.archived;
                        if (nextFlags.archived) {
                            nextFlags.oldCv = false;
                            nextFlags.deleted = false;
                        }
                    });
                };

                card.onclick = () => selectOffer(o.job_id);
                groupContainer.appendChild(card);
            });
        });

        if (visibleArchivedOffers.length) {
            const archiveHeader = document.createElement('div');
            archiveHeader.className = 'sidebar-section-header with-border';

            const archiveTitle = document.createElement('span');
            archiveTitle.className = 'sidebar-section-title';
            archiveTitle.textContent = state.i18n.translations[state.i18n.current].archive;

            const badge = document.createElement('span');
            badge.className = 'sidebar-section-badge';
            badge.textContent = visibleArchivedOffers.length;

            archiveHeader.appendChild(archiveTitle);
            archiveHeader.appendChild(badge);
            list.appendChild(archiveHeader);

            visibleArchivedOffers.forEach((o) => {
                const isActive = state.activeJobId === o.job_id;
                const card = offerRender.createOfferCard(o, {
                    isActive,
                    isLocked: false,
                    isArchived: true,
                    hasFlag: true,
                    archivedView: true,
                });

                card.querySelector('[data-action="restore-inbox"]').onclick = (event) => {
                    event.stopPropagation();
                    mutateOfferFlags(o.job_id, (nextFlags) => {
                        nextFlags.archived = false;
                        nextFlags.oldCv = false;
                        nextFlags.deleted = false;
                    });
                };

                card.querySelector('[data-action="send-old"]').onclick = (event) => {
                    event.stopPropagation();
                    mutateOfferFlags(o.job_id, (nextFlags) => {
                        nextFlags.oldCv = true;
                        nextFlags.archived = false;
                        nextFlags.deleted = false;
                    });
                };

                card.onclick = () => selectOffer(o.job_id);
                list.appendChild(card);
            });
        }

    } catch (e) {
        console.error('Impossible de charger les offres', e);
    }
}

function mutateOfferFlags(jobId, mutate) {
    const nextFlags = { ...(state.offerFlags[jobId] || {}) };
    mutate(nextFlags);
    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete state.offerFlags[jobId];
    else state.offerFlags[jobId] = nextFlags;
    state.saveOfferFlags();
    loadOffers();
}

function selectOffer(jobId) {
    state.setActiveJobId(jobId);
    emit(EVENTS.OFFER_SELECTED, { jobId });
}

function toggleOfferCategory(category) {
    const index = state.collapsedOfferCategories.indexOf(category);
    if (index >= 0) state.collapsedOfferCategories.splice(index, 1);
    else state.collapsedOfferCategories.push(category);
    state.saveCollapsedCategories();
    loadOffers();
}

function renderDashboardApplications(offers) {
    const panel = document.getElementById('dashboard-applications-panel');
    const list = document.getElementById('dashboard-applications-list');
    if (!panel || !list) return;
    const items = offers.filter((offer) => {
        const flags = state.offerFlags[offer.job_id] || {};
        return !flags.archived && !flags.oldCv && !flags.deleted;
    });

    list.innerHTML = '';
    if (!items.length) {
        panel.style.display = 'none';
        return;
    }

    panel.style.display = 'block';
    items.forEach((offer) => {
        const flags = state.offerFlags[offer.job_id] || {};
        const isArchived = !!flags.archived;
        const item = document.createElement('div');
        item.className = 'old-offer-item';
        item.style.cursor = 'pointer';
        const row = document.createElement('div');
        row.className = 'old-offer-row';

        const text = document.createElement('div');
        text.className = 'old-offer-text';

        const title = document.createElement('div');
        title.className = 'old-offer-title';
        title.textContent = offer.title;

        const company = document.createElement('div');
        company.className = 'old-offer-company';
        company.textContent = offer.entreprise || '';

        text.appendChild(title);
        text.appendChild(company);

        const actions = document.createElement('div');
        actions.className = 'old-offer-actions';

        const archiveAction = offerRender.createOfferActionButton({
            active: isArchived,
            action: 'archive',
            ariaLabel: "Archiver l'offre",
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        archiveAction.button.onclick = (event) => {
            event.stopPropagation();
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.archived = !nextFlags.archived;
            if (nextFlags.archived) {
                nextFlags.oldCv = false;
                nextFlags.deleted = false;
            }
            if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete state.offerFlags[offer.job_id];
            else state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            loadOffers();
        };

        actions.appendChild(archiveAction.wrapper);
        row.appendChild(text);
        row.appendChild(actions);
        item.appendChild(row);

        item.onclick = () => {
            state.setActiveJobId(offer.job_id);
            router.switchView('app');
            loadOffers();
            updateIframe();
        };
        list.appendChild(item);
    });
}

function renderOldOffers(offers) {
    const panel = document.getElementById('old-offers-panel');
    const list = document.getElementById('old-offers-list');
    if (!panel || !list) return;
    const oldOffers = offers.filter((offer) => state.offerFlags[offer.job_id]?.oldCv && !state.offerFlags[offer.job_id]?.deleted);

    list.innerHTML = '';
    if (!oldOffers.length) {
        panel.style.display = 'none';
        return;
    }

    panel.style.display = 'block';
    oldOffers.forEach((offer) => {
        const item = document.createElement('div');
        item.className = 'old-offer-item';
        const row = document.createElement('div');
        row.className = 'old-offer-row';

        const text = document.createElement('div');
        text.className = 'old-offer-text';

        const title = document.createElement('div');
        title.className = 'old-offer-title';
        title.textContent = offer.title;

        const company = document.createElement('div');
        company.className = 'old-offer-company';
        company.textContent = offer.entreprise || '';

        text.appendChild(title);
        text.appendChild(company);

        const actions = document.createElement('div');
        actions.className = 'old-offer-actions';

        const restoreAction = offerRender.createOfferActionButton({
            active: true,
            action: 'restore-archive',
            ariaLabel: 'Restaurer dans archive',
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        restoreAction.button.onclick = () => {
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.archived = true;
            nextFlags.oldCv = false;
            nextFlags.deleted = false;
            state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            loadOffers();
        };

        const deleteAction = offerRender.createOfferActionButton({
            active: true,
            action: 'delete',
            ariaLabel: 'Supprimer définitivement',
            iconPath: 'm14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.11 0 0 0-7.5 0',
        });
        deleteAction.button.onclick = () => {
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.deleted = true;
            nextFlags.oldCv = false;
            nextFlags.archived = false;
            state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            if (state.activeJobId === offer.job_id) state.setActiveJobId(null);
            loadOffers();
        };

        actions.appendChild(restoreAction.wrapper);
        actions.appendChild(deleteAction.wrapper);
        row.appendChild(text);
        row.appendChild(actions);
        item.appendChild(row);
        list.appendChild(item);
    });
}

function renderDashboardTreatedOffers(offers) {
    const panel = document.getElementById('dashboard-treated-panel');
    const list = document.getElementById('dashboard-treated-list');
    if (!panel || !list) return;
    const items = offers.filter((offer) => {
        const flags = state.offerFlags[offer.job_id] || {};
        return flags.archived && !flags.oldCv && !flags.deleted;
    });

    list.innerHTML = '';
    if (!items.length) {
        panel.style.display = 'none';
        return;
    }

    panel.style.display = 'block';
    items.forEach((offer) => {
        const item = document.createElement('div');
        item.className = 'old-offer-item';
        item.style.cursor = 'pointer';
        const row = document.createElement('div');
        row.className = 'old-offer-row';

        const text = document.createElement('div');
        text.className = 'old-offer-text';

        const title = document.createElement('div');
        title.className = 'old-offer-title';
        title.textContent = offer.title;

        const company = document.createElement('div');
        company.className = 'old-offer-company';
        company.textContent = offer.entreprise || '';

        text.appendChild(title);
        text.appendChild(company);

        const actions = document.createElement('div');
        actions.className = 'old-offer-actions';

        const restoreAction = offerRender.createOfferActionButton({
            active: true,
            action: 'restore-inbox',
            ariaLabel: 'Restaurer dans inbox',
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        restoreAction.button.onclick = (event) => {
            event.stopPropagation();
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.archived = false;
            nextFlags.oldCv = false;
            nextFlags.deleted = false;
            if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete state.offerFlags[offer.job_id];
            else state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            loadOffers();
        };

        const sendOldAction = offerRender.createOfferActionButton({
            active: true,
            action: 'send-old',
            ariaLabel: 'Archiver définitivement',
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        sendOldAction.button.onclick = (event) => {
            event.stopPropagation();
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.oldCv = true;
            nextFlags.archived = false;
            nextFlags.deleted = false;
            state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            loadOffers();
        };

        actions.appendChild(restoreAction.wrapper);
        actions.appendChild(sendOldAction.wrapper);
        row.appendChild(text);
        row.appendChild(actions);
        item.appendChild(row);

        item.onclick = () => {
            state.setActiveJobId(offer.job_id);
            router.switchView('app');
            loadOffers();
            updateIframe();
        };
        list.appendChild(item);
    });
}

async function updateIframe(options = {}) {
    if (!state.activeJobId) {
        iframeRender.resetIframeToEmptyState();
        return;
    }
    const { syncChatHistory = true } = options;

    const offerSlug = state.activeJobId;
    const activeTab = state.activeTab;
    const iframe = document.getElementById('iframe-doc');
    if (!iframe) return;

    const path = activeTab === 'restitution'
        ? '/restitution/index.html'
        : (activeTab === 'resume' ? '/resume/index.html' : '/cover-letter/index.html');

    let instanceSlug = window.activeInstanceSlug || offerSlug;

    if (!offerSlug || offerSlug === 'null') return;

    if (window.activeResolvedOfferSlug !== offerSlug || !window.activeInstanceSlug) {
        try {
            const res = await fetch(`/api/offres/${offerSlug}/instance`);
            if (res.ok) {
                const instance = await res.json();
                if (instance && instance.slug) {
                    instanceSlug = instance.slug;
                    window.activeInstanceData = instance;
                }
            } else if (res.status === 404) {
                // Instance non générée, c'est normal pour une nouvelle offre
                instanceSlug = null;
                window.activeInstanceData = null;
            }
        } catch (error) {
            // Erreur réseau uniquement
        }
    }

    if (state.activeJobId !== offerSlug || state.activeTab !== activeTab) return;

    window.activeResolvedOfferSlug = offerSlug;
    window.activeInstanceSlug = instanceSlug;
    if (!window.activeInstanceData?.slug || window.activeInstanceData.slug !== instanceSlug) {
        window.activeInstanceData = window.activeInstanceData && window.activeInstanceData.slug === instanceSlug
            ? window.activeInstanceData
            : { id: instanceSlug, slug: instanceSlug };
    }
    const query = activeTab === 'restitution'
        ? `offer=${encodeURIComponent(offerSlug)}&instance=${encodeURIComponent(instanceSlug)}`
        : `id=${encodeURIComponent(instanceSlug)}&offer=${encodeURIComponent(offerSlug)}`;
    iframe.removeAttribute('srcdoc');
    iframe.src = `${path}?${query}&v=${Date.now()}`;
    window.activeJobId = offerSlug; // Expose for chat.js
    if (syncChatHistory && typeof window.loadChatHistory === 'function') {
        window.loadChatHistory();
    }
    router.updatePath();
}

function renderAiChatAttachments() {
    const container = document.getElementById('ai-chat-attachments');
    if (!container) return;
    const t = state.i18n.translations[state.i18n.current];
    clear(container);

    if (!state.aiChatAttachments.length) {
        container.style.display = 'none';
        return;
    }

    container.style.display = 'flex';
    state.aiChatAttachments.forEach((file, index) => {
        const remove = document.createElement('button');
        remove.type = 'button';
        remove.className = 'ai-attachment-remove';
        remove.setAttribute('aria-label', t.attached_files);
        remove.innerText = '×';

        remove.onclick = () => {
            state.aiChatAttachments.splice(index, 1);
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
    // Attach Listeners first so they are ready for programmatic calls (like router.handleRouting)
    attachEventListeners();

    await state.loadI18n();
    await loadProfile();
    await loadOffers();
    await router.handleRouting();
    renderAiChatAttachments();
}

function attachEventListeners() {
    const safeClick = (id, fn) => { const el = document.getElementById(id); if (el) el.onclick = fn; };

    safeClick('nav-dashboard', (e) => { e.preventDefault(); router.switchView('ingest'); });
    safeClick('nav-app', (e) => { e.preventDefault(); router.switchView('app'); });
    safeClick('nav-profile', (e) => { e.preventDefault(); router.switchView('profile'); });

    safeClick('btn-save-profile', async () => {
        const btn = document.getElementById('btn-save-profile');
        btn.disabled = true;
        btn.textContent = '...';
        try {
            const data = {
                profile: {
                    firstname: document.getElementById('prof-firstname').value,
                    lastname: document.getElementById('prof-lastname').value,
                    title: document.getElementById('prof-title').value,
                    offer_type: document.getElementById('prof-offer-type').value,
                    pitch: document.getElementById('prof-pitch').value,
                    location: document.getElementById('prof-location').value,
                    phone: document.getElementById('prof-phone').value,
                    email: document.getElementById('prof-email').value,
                    linkedin: document.getElementById('prof-linkedin').value,
                    website: document.getElementById('prof-website').value,
                    github: document.getElementById('prof-github').value,
                    image: document.getElementById('prof-image-base64').value || state.loadedProfileImage || "",
                },
                apprenticeship: {
                    duration: document.getElementById('prof-duration').value,
                    rhythm: document.getElementById('prof-rhythm').value,
                },
                experiences: Array.from(document.querySelectorAll('#list-experiences .form-row-exp')).map(r => ({
                    role: r.querySelector('.exp-role').value,
                    company: r.querySelector('.exp-company').value,
                    period: r.querySelector('.exp-period').value,
                    description: r.querySelector('.exp-desc').value.split('\n').filter(Boolean),
                })),
                projects: Array.from(document.querySelectorAll('#list-projects .form-row-exp')).map(r => ({
                    role: r.querySelector('.exp-role').value,
                    company: r.querySelector('.exp-company').value,
                    period: r.querySelector('.exp-period').value,
                    description: r.querySelector('.exp-desc').value.split('\n').filter(Boolean),
                })),
                education: Array.from(document.querySelectorAll('.form-row-edu')).map(r => ({
                    school: r.querySelector('.edu-school').value,
                    degree: r.querySelector('.edu-degree').value,
                    period: r.querySelector('.edu-period').value,
                })),
                languages: Array.from(document.querySelectorAll('.form-row-lang')).map(r => ({
                    name: r.querySelector('.lang-name').value,
                    level: r.querySelector('.lang-level').value,
                })),
                skills: Array.from(document.querySelectorAll('.skill-cat-row')).map(r => ({
                    category: r.querySelector('.skill-cat-name').value,
                    items: Array.from(r.querySelectorAll('.skill-text')).map(s => s.textContent),
                })),
                documents: {
                    resume_template: JSON.parse(document.getElementById('prof-resume-template').value || "{}"),
                    cover_letter_template: JSON.parse(document.getElementById('prof-cover-letter-template').value || "{}"),
                },
                ...state.loadedProfileExtras
            };
            await api.saveProfile(data);

            const annexeRows = Array.from(document.querySelectorAll('.form-row-annexe'));
            for (const row of annexeRows) {
                if (row.dataset.markedForDeletion === "true") {
                    await api.deleteAnnexe(row.dataset.fileId);
                } else if (!row.dataset.fileId && row.dataset.fileData) {
                    await api.uploadAnnexe({
                        label: row.querySelector('.annexe-name').value,
                        filename: row.dataset.fileName,
                        content_type: row.dataset.fileType,
                        data_url: row.dataset.fileData
                    });
                } else if (row.dataset.fileId) {
                    await api.updateAnnexe(row.dataset.fileId, {
                        label: row.querySelector('.annexe-name').value
                    });
                }
            }

            await loadProfile();
            alert('Profil sauvegardé !');
        } catch (e) { alert('Erreur sauvegarde'); console.error(e); }
        finally { btn.disabled = false; btn.textContent = 'Sauvegarder'; }
    });

    const profPreview = document.getElementById('prof-image-preview');
    if (profPreview) profPreview.onclick = () => document.getElementById('prof-image-file').click();
    const profFile = document.getElementById('prof-image-file');
    if (profFile) profFile.onchange = async (e) => {
        const file = e.target.files[0];
        if (!file) return;
        const b64 = await ui.readFileAsDataUrl(file);
        state.setLoadedProfileImage(b64);
        const b64Input = document.getElementById('prof-image-base64');
        if (b64Input) b64Input.value = b64;
        profPreview.style.backgroundImage = `url(${b64})`;
        const placeholder = document.getElementById('preview-placeholder');
        if (placeholder) placeholder.style.display = 'none';
    };

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
            state.setActiveTab(btn.dataset.target);
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

    safeClick('btn-ingest-run', async () => {
        const input = document.getElementById('job-input');
        if (!input) return;
        const rawText = input.value;
        if (!rawText) return;

        const btn = document.getElementById('btn-ingest-run');
        btn.disabled = true;
        const oldText = btn.innerHTML;
        btn.innerHTML = '...';

        try {
            const ingestRes = await api.ingestOffer(rawText);
            if (!ingestRes.job_id) throw new Error("Échec ingestion");
            state.setActiveJobId(ingestRes.job_id);

            const delivs = document.getElementById('deliv-selector-ingest');
            const options = {
                restitution: delivs?.querySelector('[data-deliv="restitution"]')?.classList.contains('active') ?? true,
                resume: delivs?.querySelector('[data-deliv="resume"]')?.classList.contains('active') ?? true,
                cover_letter: delivs?.querySelector('[data-deliv="cover"]')?.classList.contains('active') ?? true,
            };

            const genRes = await api.generateApplication(ingestRes.job_id, state.selectedLlmProvider, options);
            router.switchView('app');
            loadOffers();
            if (genRes.slug) updateIframe();
        } catch (e) {
            alert('Erreur: ' + e.message);
        } finally {
            btn.disabled = false;
            btn.innerHTML = oldText;
        }
    });

    safeClick('ai-chat-attach-btn', () => document.getElementById('ai-chat-file-input').click());
    const chatFile = document.getElementById('ai-chat-file-input');
    if (chatFile) chatFile.onchange = async (e) => {
        const files = Array.from(e.target.files);
        for (const file of files) {
            const data = await ui.readFileAsDataUrl(file);
            state.aiChatAttachments.push({ name: file.name, content_type: file.type, data });
        }
        renderAiChatAttachments();
        e.target.value = '';
    };
}

function setupSelector(containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    // 1. Initial State Sync (DOM -> State or State -> DOM)
    container.querySelectorAll('.llm-pill').forEach(pill => {
        const prov = pill.dataset.provider;
        const deliv = pill.dataset.deliv;

        if (prov) {
            if (state.selectedLlmProvider === prov) pill.classList.add('active');
            else pill.classList.remove('active');
        } else if (deliv) {
            // Mapping UI "restitution" to internal state "restitution" etc.
            // Note: cover in UI vs cover_letter in state (wait, state uses cover from localStorage)
            // Let's check state.js delivConfig keys
            const val = state.delivConfig[deliv];
            if (val === true) pill.classList.add('active');
            else if (val === false) pill.classList.remove('active');
        }

        // 2. Click Handler
        pill.onclick = () => {
            if (prov) {
                container.querySelectorAll('.llm-pill').forEach(p => p.classList.remove('active'));
                pill.classList.add('active');
                state.setSelectedLlmProvider(prov);
            } else if (deliv) {
                pill.classList.toggle('active');
                state.setDelivConfig(deliv, pill.classList.contains('active'));
            }
        };
    });
}

document.addEventListener('DOMContentLoaded', init);
