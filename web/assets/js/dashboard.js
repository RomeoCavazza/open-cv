import * as state from './state.js';
import * as api from './api.js';
import * as ui from './ui.js';

// --- Dashboard Logic ---

const views = { 
    ingest: document.getElementById('view-ingest'), 
    app: document.getElementById('view-app'), 
    profile: document.getElementById('view-profile') 
};

function switchView(viewName) {
    Object.values(views).forEach(v => {
        v.classList.remove('active');
        v.scrollTop = 0;
    });
    views[viewName].classList.add('active');
    views[viewName].scrollTop = 0;
    document.querySelectorAll('.nav-link').forEach(l => l.classList.remove('active'));
    if (viewName === 'ingest') document.getElementById('nav-dashboard').classList.add('active');
    if (viewName === 'app') { document.getElementById('nav-app').classList.add('active'); loadOffers(); }
    if (viewName === 'profile') document.getElementById('nav-profile').classList.add('active');
    updatePath();
}

// Expose for legacy scripts (chat.js)
window.updateIframe = updateIframe;

function updatePath() {
    let path = '/';
    if (views.app.classList.contains('active')) {
        path = '/applications';
        if (state.activeJobId) {
            path += `/${state.activeJobId}`;
            if (state.activeTab) path += `/${state.activeTab}`;
        }
    } else if (views.profile.classList.contains('active')) {
        path = '/profil';
    }
    if (window.location.pathname !== path) {
        history.pushState(null, null, path);
    }
}

async function handleRouting() {
    const path = window.location.pathname;
    if (!path || path === '/') {
        switchView('ingest');
        return;
    }

    const parts = path.split('/').filter(Boolean); // applications, slug, tab
    if (parts[0] === 'applications') {
        switchView('app');
        if (parts[1]) {
            state.setActiveJobId(parts[1]);
            if (parts[2]) state.setActiveTab(parts[2]);
            
            await loadOffers();
            
            if (state.activeTab) {
                const tab = document.querySelector(`.tab[data-target="${state.activeTab}"]`);
                if (tab) tab.click();
            }
        }
    } else if (parts[0] === 'profil') {
        switchView('profile');
    }
}

window.addEventListener('popstate', handleRouting);

async function loadProfile() {
    try {
        const profil = await api.fetchProfile();
        state.setActiveProfilId(profil.id || null);
        const content = profil.content;
        
        state.setLoadedProfileExtras(Object.fromEntries(
            Object.entries(content).filter(([key]) => ![
                'profile', 'apprenticeship', 'experiences', 'projects', 
                'education', 'languages', 'skills', 'labels'
            ].includes(key))
        ));

        document.getElementById('prof-firstname').value = content.profile?.firstname || "";
        document.getElementById('prof-lastname').value = content.profile?.lastname || "";
        document.getElementById('prof-title').value = content.profile?.title || "";
        document.getElementById('prof-offer-type').value = content.profile?.offer_type || "Alternance";
        document.getElementById('prof-duration').value = content.apprenticeship?.duration || "";
        document.getElementById('prof-rhythm').value = content.apprenticeship?.rhythm || "";
        document.getElementById('prof-pitch').value = content.profile?.pitch || "";
        document.getElementById('prof-location').value = content.profile?.location || "";
        document.getElementById('prof-phone').value = content.profile?.phone || "";
        document.getElementById('prof-email').value = content.profile?.email || "";
        document.getElementById('prof-linkedin').value = content.profile?.linkedin || "";
        document.getElementById('prof-website').value = content.profile?.website || "";
        document.getElementById('prof-github').value = content.profile?.github || "";
        
        document.getElementById('prof-resume-template').value = ui.stringifyDocument(content.documents?.resume_template || content.resume_template);
        document.getElementById('prof-cover-letter-template').value = ui.stringifyDocument(content.documents?.cover_letter_template || content.cover_letter_template);
        
        state.setLoadedApprenticeshipCalendarDocument(content.documents?.apprenticeship_calendar || null);
        updateCalendarDocumentUI(state.loadedApprenticeshipCalendarDocument);
        
        state.setLoadedProfileImage(content.profile?.image || "");
        document.getElementById('prof-image-base64').value = state.loadedProfileImage;
        
        if (content.profile?.image) {
            document.getElementById('prof-image-preview').style.backgroundImage = `url(${content.profile.image})`;
            document.getElementById('preview-placeholder').style.display = 'none';
        } else {
            document.getElementById('prof-image-preview').style.backgroundImage = '';
            document.getElementById('preview-placeholder').style.display = 'block';
        }

        ui.renderList('list-experiences', content.experiences || [], ui.createExpRow);
        ui.renderList('list-projects', content.projects || [], ui.createExpRow);
        ui.renderList('list-education', content.education || [], ui.createEduRow);
        ui.renderList('list-languages', content.languages || [], ui.createLangRow);
        ui.renderList('list-skills', content.skills || [], ui.createSkillRow);
        
        const docs = (content.documents && typeof content.documents === 'object') ? content.documents : {};
        ui.renderList('list-annexes', docs.annexes || [], ui.createAnnexeRow);
        
        ui.updateUIStrings();
    } catch (e) { console.warn("Profile load failed", e); }
}

function updateCalendarDocumentUI(documentValue) {
    const preview = document.getElementById('prof-apprenticeship-calendar-preview');
    const legacy = document.getElementById('prof-apprenticeship-calendar-legacy');
    const legacyValue = document.getElementById('prof-apprenticeship-calendar-legacy-value');
    
    if (!documentValue) {
        legacy.style.display = 'none';
        preview.src = '/api/profile/active/calendar';
        preview.style.display = 'block';
        return;
    }

    if (typeof documentValue === 'string') {
        preview.style.display = 'none';
        legacy.style.display = 'block';
        legacyValue.value = documentValue;
        return;
    }

    if (documentValue.data_url) {
        preview.style.display = 'block';
        preview.src = documentValue.data_url;
    }
}

async function loadOffers() {
    try {
        const data = await api.fetchOffers();
        const offers = data.entries || [];
        
        renderDashboardApplications(offers);
        renderDashboardTreatedOffers(offers);
        renderOldOffers(offers);
        
        const list = document.getElementById('offers-list');
        list.innerHTML = '';
        const inboxLabel = state.i18n.translations[state.i18n.current].inbox;
        
        const visibleInboxOffers = offers.filter((offer) => {
            const flags = state.offerFlags[offer.job_id] || {};
            return !flags.archived && !flags.oldCv && !flags.deleted;
        });
        
        const visibleArchivedOffers = offers.filter((offer) => {
            const flags = state.offerFlags[offer.job_id] || {};
            return flags.archived && !flags.oldCv && !flags.deleted;
        });
        
        document.getElementById('sidebar-inbox-header').textContent = `${inboxLabel} (${visibleInboxOffers.length})`;
        
        const groups = {};
        visibleInboxOffers.forEach(o => {
            const cat = o.category || "AUTRES";
            if (!groups[cat]) groups[cat] = [];
            groups[cat].push(o);
        });

        Object.keys(groups).sort().forEach(cat => {
            const isDefault = cat === "AUTRES" || cat.toUpperCase().includes("INBOX");
            const isCollapsed = state.collapsedOfferCategories.includes(cat);
            let groupContainer = list;
            
            if (!isDefault) {
                const catDiv = document.createElement('div');
                catDiv.className = `offer-group-header${isCollapsed ? ' collapsed' : ''}`;
                catDiv.innerHTML = `<span class="offer-group-toggle">${
                    isCollapsed
                        ? `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 13.5H9m4.06-7.19-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" /></svg>`
                        : `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 10.5v6m3-3H9m4.06-7.19-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" /></svg>`
                }</span><span>${cat} (${groups[cat].length})</span>`;
                
                catDiv.onclick = () => {
                    if (state.collapsedOfferCategories.includes(cat)) {
                        state.collapsedOfferCategories.splice(state.collapsedOfferCategories.indexOf(cat), 1);
                    } else {
                        state.collapsedOfferCategories.push(cat);
                    }
                    state.saveCollapsedCategories();
                    loadOffers();
                };
                list.appendChild(catDiv);
                groupContainer = document.createElement('div');
                groupContainer.style.display = isCollapsed ? 'none' : 'block';
                list.appendChild(groupContainer);
            }
            
            groups[cat].forEach(o => {
                const card = document.createElement('div');
                const isActive = state.activeJobId === o.job_id;
                const flags = state.offerFlags[o.job_id] || {};
                const isLocked = !!flags.locked;
                const isArchived = !!flags.archived;
                const hasFlag = isLocked || !!flags.archived;
                
                card.className = `offer-card ${isActive ? 'active' : ''} ${hasFlag ? 'has-flag' : ''} ${isArchived ? 'is-archived' : ''}`;
                card.style = `padding: 12px 16px; cursor: pointer; border-radius: 8px; margin: 4px 8px; transition: all 0.2s; background: ${isActive ? 'white' : 'transparent'};`;
                card.innerHTML = `<div class="offer-card-inner"><div class="offer-card-text"><div style="display:flex; align-items:flex-start; gap:8px;"><div class="offer-title" style="flex:1;">${o.title}</div><div class="offer-actions-slot"><span class="offer-action-visibility ${isLocked ? 'is-active' : ''}"><button type="button" class="offer-action-btn ${isLocked ? 'is-active' : ''}" data-action="lock" aria-label="Verrouiller l'offre"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" /></svg></button></span><span class="offer-action-visibility ${isArchived ? 'is-active' : ''}"><button type="button" class="offer-action-btn ${isArchived ? 'is-active' : ''}" data-action="archive" aria-label="Archiver l'offre"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></span></div></div><div class="offer-company">${o.entreprise || ""}</div></div></div>`;
                
                card.querySelector('[data-action="lock"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(state.offerFlags[o.job_id] || {}) };
                    nextFlags.locked = !nextFlags.locked;
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete state.offerFlags[o.job_id];
                    else state.offerFlags[o.job_id] = nextFlags;
                    state.saveOfferFlags();
                    loadOffers();
                };
                
                card.querySelector('[data-action="archive"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(state.offerFlags[o.job_id] || {}) };
                    nextFlags.archived = !nextFlags.archived;
                    if (nextFlags.archived) {
                        nextFlags.oldCv = false;
                        nextFlags.deleted = false;
                    }
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete state.offerFlags[o.job_id];
                    else state.offerFlags[o.job_id] = nextFlags;
                    state.saveOfferFlags();
                    loadOffers();
                };
                
                card.onclick = () => { state.setActiveJobId(o.job_id); loadOffers(); updateIframe(); };
                groupContainer.appendChild(card);
            });
        });

        if (visibleArchivedOffers.length) {
            const archiveHeader = document.createElement('div');
            archiveHeader.className = 'sidebar-header';
            archiveHeader.style.padding = '24px 24px 12px';
            archiveHeader.style.marginTop = '16px';
            archiveHeader.style.borderBottom = 'none';
            archiveHeader.innerHTML = `<h2>${state.i18n.translations[state.i18n.current].archive} (${visibleArchivedOffers.length})</h2>`;
            list.appendChild(archiveHeader);

            visibleArchivedOffers.forEach((o) => {
                const card = document.createElement('div');
                const isActive = state.activeJobId === o.job_id;
                card.className = `offer-card ${isActive ? 'active' : ''} is-archived archive-muted`;
                card.style = `padding: 12px 16px; cursor: pointer; border-radius: 8px; margin: 4px 8px; transition: all 0.2s; background: ${isActive ? 'white' : 'transparent'};`;
                card.innerHTML = `<div class="offer-card-inner"><div class="offer-card-text"><div style="display:flex; align-items:flex-start; gap:8px;"><div class="offer-title" style="flex:1;">${o.title}</div><div class="offer-actions-slot"><span class="offer-action-visibility is-active"><button type="button" class="offer-action-btn" data-action="restore-inbox" aria-label="Restaurer dans inbox"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></span><span class="offer-action-visibility is-active"><button type="button" class="offer-action-btn" data-action="send-old" aria-label="Retirer de la sidebar"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></span></div></div><div class="offer-company">${o.entreprise || ''}</div></div></div>`;
                
                card.querySelector('[data-action="restore-inbox"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(state.offerFlags[o.job_id] || {}) };
                    nextFlags.archived = false;
                    nextFlags.oldCv = false;
                    nextFlags.deleted = false;
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete state.offerFlags[o.job_id];
                    else state.offerFlags[o.job_id] = nextFlags;
                    state.saveOfferFlags();
                    loadOffers();
                };
                
                card.querySelector('[data-action="send-old"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(state.offerFlags[o.job_id] || {}) };
                    nextFlags.oldCv = true;
                    nextFlags.archived = false;
                    nextFlags.deleted = false;
                    state.offerFlags[o.job_id] = nextFlags;
                    state.saveOfferFlags();
                    loadOffers();
                };
                
                card.onclick = () => { state.setActiveJobId(o.job_id); loadOffers(); updateIframe(); };
                list.appendChild(card);
            });
        }

        const fallbackActive = visibleInboxOffers[0] || visibleArchivedOffers[0];
        if (!state.activeJobId && fallbackActive) { state.setActiveJobId(fallbackActive.job_id); updateIframe(); loadOffers(); }
    } catch (e) {}
}

function renderDashboardApplications(offers) {
    const panel = document.getElementById('dashboard-applications-panel');
    const list = document.getElementById('dashboard-applications-list');
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
        item.innerHTML = `<div class="old-offer-row"><div class="old-offer-text"><div class="old-offer-title">${offer.title}</div><div class="old-offer-company">${offer.entreprise || ''}</div></div><div class="old-offer-actions"><span class="offer-action-visibility"><button type="button" class="offer-action-btn" data-action="edit" aria-label="Éditer l'offre"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" /></svg></button></span><span class="offer-action-visibility ${isArchived ? 'is-active' : ''}"><button type="button" class="offer-action-btn ${isArchived ? 'is-active' : ''}" data-action="archive" aria-label="Archiver l'offre"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></span></div></div>`;
        
        item.querySelector('[data-action="edit"]').onclick = (event) => {
            event.stopPropagation();
            state.setActiveJobId(offer.job_id);
            switchView('app');
            loadOffers();
            updateIframe();
        };
        
        item.querySelector('[data-action="archive"]').onclick = (event) => {
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
        
        item.onclick = () => {
            state.setActiveJobId(offer.job_id);
            switchView('app');
            loadOffers();
            updateIframe();
        };
        list.appendChild(item);
    });
}

function renderOldOffers(offers) {
    const panel = document.getElementById('old-offers-panel');
    const list = document.getElementById('old-offers-list');
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
        item.innerHTML = `<div class="old-offer-row"><div class="old-offer-text"><div class="old-offer-title">${offer.title}</div><div class="old-offer-company">${offer.entreprise || ''}</div></div><div class="old-offer-actions"><button type="button" class="offer-action-btn" data-action="restore-archive" aria-label="Restaurer dans archive"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button><button type="button" class="offer-action-btn" data-action="delete" aria-label="Supprimer définitivement"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.11 0 0 0-7.5 0" /></svg></button></div></div>`;
        
        item.querySelector('[data-action="restore-archive"]').onclick = () => {
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.archived = true;
            nextFlags.oldCv = false;
            nextFlags.deleted = false;
            state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            loadOffers();
        };
        
        item.querySelector('[data-action="delete"]').onclick = () => {
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.deleted = true;
            nextFlags.oldCv = false;
            nextFlags.archived = false;
            state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            if (state.activeJobId === offer.job_id) state.setActiveJobId(null);
            loadOffers();
        };
        list.appendChild(item);
    });
}

function renderDashboardTreatedOffers(offers) {
    const panel = document.getElementById('dashboard-treated-panel');
    const list = document.getElementById('dashboard-treated-list');
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
        item.innerHTML = `<div class="old-offer-row"><div class="old-offer-text"><div class="old-offer-title">${offer.title}</div><div class="old-offer-company">${offer.entreprise || ''}</div></div><div class="old-offer-actions"><button type="button" class="offer-action-btn" data-action="restore-inbox" aria-label="Restaurer dans inbox"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button><button type="button" class="offer-action-btn" data-action="send-old" aria-label="Archiver définitivement"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></div></div>`;
        
        item.querySelector('[data-action="restore-inbox"]').onclick = (event) => {
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
        
        item.querySelector('[data-action="send-old"]').onclick = (event) => {
            event.stopPropagation();
            const nextFlags = { ...(state.offerFlags[offer.job_id] || {}) };
            nextFlags.oldCv = true;
            nextFlags.archived = false;
            nextFlags.deleted = false;
            state.offerFlags[offer.job_id] = nextFlags;
            state.saveOfferFlags();
            loadOffers();
        };
        
        item.onclick = () => {
            state.setActiveJobId(offer.job_id);
            switchView('app');
            loadOffers();
            updateIframe();
        };
        list.appendChild(item);
    });
}

function updateIframe() {
    if (!state.activeJobId) return;
    const iframe = document.getElementById('iframe-doc');
    const path = state.activeTab === 'restitution' ? '/restitution/index.html' : (state.activeTab === 'resume' ? '/resume/index.html' : '/cover-letter/index.html');
    iframe.src = `${path}?id=${state.activeJobId}&v=${Date.now()}`;
    window.activeJobId = state.activeJobId; // Expose for chat.js
    updatePath();
}

function renderAiChatAttachments() {
    const container = document.getElementById('ai-chat-attachments');
    const t = state.i18n.translations[state.i18n.current];
    container.innerHTML = '';

    if (!state.aiChatAttachments.length) {
        container.style.display = 'none';
        return;
    }

    container.style.display = 'flex';
    state.aiChatAttachments.forEach((file, index) => {
        const chip = document.createElement('div');
        chip.className = 'ai-attachment-chip';
        chip.innerHTML = `
            <span class="ai-attachment-name" title="${file.name}">${file.name}</span>
            <button type="button" class="ai-attachment-remove" aria-label="${t.attached_files}">×</button>
        `;
        chip.querySelector('.ai-attachment-remove').onclick = () => {
            state.aiChatAttachments.splice(index, 1);
            renderAiChatAttachments();
        };
        container.appendChild(chip);
    });
}

function syncLlmSelectors(provider) {
    state.setSelectedLlmProvider(provider);
    document.querySelectorAll('.llm-pill[data-provider]').forEach(p => {
        if (p.dataset.provider === provider) p.classList.add('active');
        else p.classList.remove('active');
    });
}

// --- Event Listeners ---

document.querySelectorAll('.lang-toggle').forEach(sel => {
    sel.onchange = (e) => {
        state.i18n.current = e.target.value;
        document.querySelectorAll('.lang-toggle').forEach(s => s.value = state.i18n.current);
        ui.updateUIStrings();
        renderAiChatAttachments();
        loadOffers();
    };
});

document.getElementById('nav-dashboard').onclick = () => switchView('ingest');
document.getElementById('nav-app').onclick = () => switchView('app');
document.getElementById('nav-profile').onclick = () => switchView('profile');

document.getElementById('btn-save-profile').onclick = async () => {
    const btn = document.getElementById('btn-save-profile');
    btn.disabled = true;
    let resumeTemplate;
    let coverLetterTemplate;
    try {
        resumeTemplate = JSON.parse(document.getElementById('prof-resume-template').value || '{}');
        coverLetterTemplate = JSON.parse(document.getElementById('prof-cover-letter-template').value || '{}');
    } catch (error) {
        console.error("Profile document JSON invalid", error);
        alert("Le JSON du modèle CV ou du modèle lettre est invalide.");
        btn.disabled = false;
        return;
    }

    const content = {
        ...state.loadedProfileExtras,
        profile: {
            firstname: document.getElementById('prof-firstname').value,
            lastname: document.getElementById('prof-lastname').value,
            name: document.getElementById('prof-firstname').value + " " + document.getElementById('prof-lastname').value,
            image: document.getElementById('prof-image-base64').value || state.loadedProfileImage || "",
            title: document.getElementById('prof-title').value,
            pitch: document.getElementById('prof-pitch').value,
            location: document.getElementById('prof-location').value,
            phone: document.getElementById('prof-phone').value,
            email: document.getElementById('prof-email').value,
            linkedin: document.getElementById('prof-linkedin').value,
            website: document.getElementById('prof-website').value,
            github: document.getElementById('prof-github').value,
            offer_type: document.getElementById('prof-offer-type').value
        },
        apprenticeship: {
            duration: document.getElementById('prof-duration').value,
            rhythm: document.getElementById('prof-rhythm').value,
            start: "septembre 2026"
        },
        experiences: Array.from(document.querySelectorAll('#list-experiences .form-row-exp')).map(row => ({
            role: row.querySelector('.exp-role').value,
            company: row.querySelector('.exp-company').value,
            period: row.querySelector('.exp-period').value,
            description: row.querySelector('.exp-desc').value.split('\n').filter(l => l.trim())
        })),
        projects: Array.from(document.querySelectorAll('#list-projects .form-row-exp')).map(row => ({
            role: row.querySelector('.exp-role').value,
            company: row.querySelector('.exp-company').value,
            period: row.querySelector('.exp-period').value,
            description: row.querySelector('.exp-desc').value.split('\n').filter(l => l.trim())
        })),
        education: Array.from(document.querySelectorAll('.form-row-edu')).map(row => ({
            school: row.querySelector('.edu-school').value,
            degree: row.querySelector('.edu-degree').value,
            period: row.querySelector('.edu-period').value
        })),
        languages: Array.from(document.querySelectorAll('.form-row-lang')).map(row => ({
            name: row.querySelector('.lang-name').value,
            level: row.querySelector('.lang-level').value
        })),
        skills: Array.from(document.querySelectorAll('.skill-cat-row')).map(row => ({
            category: row.querySelector('.skill-cat-name').value,
            items: Array.from(row.querySelectorAll('.skills-pills-container .skill-pill')).map(p => {
                const textEl = p.querySelector('.skill-text');
                return textEl ? textEl.innerText.trim() : p.innerText.replace(' ×', '').trim();
            })
        })).filter(group => group.category.trim() || group.items.length > 0),
        documents: {
            resume_template: resumeTemplate && Object.keys(resumeTemplate).length > 0 ? resumeTemplate : undefined,
            cover_letter_template: coverLetterTemplate && Object.keys(coverLetterTemplate).length > 0 ? coverLetterTemplate : undefined,
            apprenticeship_calendar: state.loadedApprenticeshipCalendarDocument || undefined,
            annexes: Array.from(document.querySelectorAll('.form-row-annexe')).map(row => ({
                label: row.querySelector('.annexe-name').value,
                name: row.dataset.fileName,
                type: row.dataset.fileType,
                data_url: row.dataset.fileData
            })).filter(a => a.label.trim() || a.name)
        },
        labels: { contact: "CONTACT", skills: "COMPÉTENCES", languages: "LANGUES", experiences: "EXPÉRIENCES", projects: "PROJETS", education: "FORMATIONS", download: "Download PDF (A4)" }
    };

    try {
        await api.saveProfile(content);
        alert("Profil sauvegardé !");
        await loadProfile();
    } catch (e) {
        console.error("Profile save failed", e);
        alert("Erreur de sauvegarde du profil.");
    } finally { 
        btn.disabled = false; 
    }
};

document.getElementById('prof-image-preview').onclick = () => document.getElementById('prof-image-file').click();
document.getElementById('ai-chat-attach-btn').onclick = () => document.getElementById('ai-chat-file-input').click();

document.getElementById('prof-image-file').onchange = async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    try {
        const b64 = await fileToOptimizedDataUrl(file);
        state.setLoadedProfileImage(b64);
        document.getElementById('prof-image-base64').value = b64;
        document.getElementById('prof-image-preview').style.backgroundImage = `url(${b64})`;
        document.getElementById('preview-placeholder').style.display = 'none';
    } catch (error) {
        alert("Image trop lourde ou illisible.");
    }
};

async function fileToOptimizedDataUrl(file) {
    const dataUrl = await ui.readFileAsDataUrl(file);
    const img = await new Promise((resolve, reject) => {
        const image = new Image();
        image.onload = () => resolve(image);
        image.onerror = () => reject(new Error('image-decode-failed'));
        image.src = dataUrl;
    });

    const maxSize = 512;
    const ratio = Math.min(1, maxSize / Math.max(img.width, img.height));
    const width = Math.max(1, Math.round(img.width * ratio));
    const height = Math.max(1, Math.round(img.height * ratio));

    const canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(img, 0, 0, width, height);

    return canvas.toDataURL('image/jpeg', 0.82);
}

document.getElementById('ai-chat-file-input').onchange = (e) => {
    const files = Array.from(e.target.files || []);
    files.forEach((file) => {
        const exists = state.aiChatAttachments.some(a => a.name === file.name && a.size === file.size);
        if (!exists) state.aiChatAttachments.push(file);
    });
    e.target.value = '';
    renderAiChatAttachments();
};

document.getElementById('btn-ingest-run').onclick = async () => {
    const inputText = document.getElementById('job-input').value.trim();
    if (!inputText) return;
    const btn = document.getElementById('btn-ingest-run');
    btn.disabled = true;

    const delivs = {};
    document.querySelectorAll('#deliv-selector-ingest .llm-pill').forEach(p => {
        delivs[p.dataset.deliv] = p.classList.contains('active');
    });

    const payload = { 
        input: inputText, 
        config: { resume: delivs.resume, cover: delivs.cover, analysis: delivs.restitution }, 
        profil_id: state.activeProfilId, 
        llm_provider: state.selectedLlmProvider 
    };

    try {
        await api.runIngest(payload);
        state.setActiveJobId(null);
        await loadOffers();
        switchView('app');
    } catch (e) {
        console.error(e);
    } finally { 
        btn.disabled = false; 
    }
};

document.querySelectorAll('.tab').forEach(tab => {
    tab.onclick = () => {
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        tab.classList.add('active');
        state.setActiveTab(tab.dataset.target);
        updateIframe();
    };
});

document.getElementById('btn-download-pdf').onclick = () => {
    const iframe = document.getElementById('iframe-doc');
    if (iframe.src) window.open(iframe.src, '_blank').print();
};

document.querySelectorAll('.llm-pill[data-provider]').forEach(pill => {
    pill.onclick = () => syncLlmSelectors(pill.dataset.provider);
});

document.querySelectorAll('#deliv-selector-ingest .llm-pill').forEach(pill => {
    pill.onclick = () => {
        pill.classList.toggle('active');
        const current = {};
        document.querySelectorAll('#deliv-selector-ingest .llm-pill').forEach(p => {
            current[p.dataset.deliv] = p.classList.contains('active');
        });
        localStorage.setItem('recruitai_delivs', JSON.stringify(current));
    };
});

document.getElementById('add-exp').onclick = () => {
    const container = document.getElementById('list-experiences');
    container.appendChild(ui.createExpRow());
    ui.initSortableContainer(container);
};
document.getElementById('add-project').onclick = () => {
    const container = document.getElementById('list-projects');
    container.appendChild(ui.createExpRow());
    ui.initSortableContainer(container);
};
document.getElementById('add-edu').onclick = () => document.getElementById('list-education').appendChild(ui.createEduRow());
document.getElementById('add-lang').onclick = () => document.getElementById('list-languages').appendChild(ui.createLangRow());
document.getElementById('add-skill-cat').onclick = () => document.getElementById('list-skills').appendChild(ui.createSkillRow());
document.getElementById('add-annexe').onclick = () => document.getElementById('prof-annexe-bulk-file').click();

document.getElementById('prof-annexe-bulk-file').onchange = async (e) => {
    const files = Array.from(e.target.files || []);
    const container = document.getElementById('list-annexes');
    for (const file of files) {
        try {
            const dataUrl = await ui.readFileAsDataUrl(file);
            const rawName = file.name.split('.')[0];
            const capitalized = rawName.charAt(0).toUpperCase() + rawName.slice(1);
            container.appendChild(ui.createAnnexeRow({ label: capitalized, name: file.name, type: file.type, data_url: dataUrl }));
        } catch (err) { console.error(err); }
    }
    e.target.value = '';
};

// Initial Load
syncLlmSelectors(state.selectedLlmProvider);
loadProfile();
loadOffers();
handleRouting();
ui.updateUIStrings();
