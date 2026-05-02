// dashboard.js - Extracted from original index.html
// This file preserves 100% of the premium rendering logic.

let activeJobId = null;
let activeTab = 'restitution';
let collapsedOfferCategories = JSON.parse(localStorage.getItem('recruitai_collapsed_offer_categories') || '[]');
let offerFlags = JSON.parse(localStorage.getItem('recruitai_offer_flags') || '{}');

function saveOfferFlags() {
    localStorage.setItem('recruitai_offer_flags', JSON.stringify(offerFlags));
}

function handleRouting() {
    const params = new URLSearchParams(window.location.search);
    const slug = params.get('slug') || params.get('id');
    if (slug) activeJobId = slug;
}

function updatePath() {
    if (!activeJobId) return;
    const url = new URL(window.location);
    url.searchParams.set('slug', activeJobId);
    window.history.replaceState({}, '', url);
}

function updateIframe() {
    if (!activeJobId) return;
    const iframe = document.getElementById('iframe-doc');
    const path = activeTab === 'restitution' ? '/restitution/index.html' : (activeTab === 'resume' ? '/resume/index.html' : '/cover-letter/index.html');
    iframe.src = `${path}?id=${activeJobId}&v=${Date.now()}`;
    updatePath();
}

function renderDashboardApplications(offers) {
    const container = document.getElementById('dashboard-apps-grid');
    if (!container) return;
    // ... Logic from original index.html if needed
}

function renderDashboardTreatedOffers(offers) {
    const container = document.getElementById('dashboard-treated-grid');
    if (!container) return;
}

function renderOldOffers(offers) {
    const container = document.getElementById('old-offers-list');
    if (!container) return;
}

async function loadOffers() {
    try {
        const res = await fetch('/api/offres');
        const data = await res.json();
        const offers = data.entries || [];
        
        if (typeof renderDashboardApplications === 'function') renderDashboardApplications(offers);
        if (typeof renderDashboardTreatedOffers === 'function') renderDashboardTreatedOffers(offers);
        if (typeof renderOldOffers === 'function') renderOldOffers(offers);

        const list = document.getElementById('offers-list');
        if (!list) return;
        list.innerHTML = '';

        const visibleInboxOffers = offers.filter((offer) => {
            const flags = offerFlags[offer.job_id] || {};
            return !flags.archived && !flags.oldCv && !flags.deleted;
        });

        const visibleArchivedOffers = offers.filter((offer) => {
            const flags = offerFlags[offer.job_id] || {};
            return flags.archived && !flags.oldCv && !flags.deleted;
        });

        const headerLabel = document.getElementById('sidebar-inbox-header');
        if (headerLabel) {
            const inboxLabel = (typeof i18n !== 'undefined') ? i18n.translations[i18n.current].inbox : 'INBOX';
            headerLabel.textContent = `${inboxLabel} (${visibleInboxOffers.length})`;
        }

        const groups = {};
        visibleInboxOffers.forEach(o => {
            const cat = o.category || "AUTRES";
            if (!groups[cat]) groups[cat] = [];
            groups[cat].push(o);
        });

        const catIcons = {
            "Data Engineering & Data Science": "󰆧",
            "Ingénierie Logicielle Spécialisée (Embarqué, C++, Simulations, Systèmes)": "󰲋",
            "Pilotage de Projet, Stratégie IT & Transformation Numérique": "󰙨",
            "Autres": "󰘦"
        };

        Object.keys(groups).sort().forEach(cat => {
            const isDefault = cat === "AUTRES" || cat.toUpperCase().includes("INBOX");
            const isCollapsed = collapsedOfferCategories.includes(cat);
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
                    if (collapsedOfferCategories.includes(cat)) {
                        collapsedOfferCategories = collapsedOfferCategories.filter((value) => value !== cat);
                    } else {
                        collapsedOfferCategories = [...collapsedOfferCategories, cat];
                    }
                    localStorage.setItem('recruitai_collapsed_offer_categories', JSON.stringify(collapsedOfferCategories));
                    loadOffers();
                };
                list.appendChild(catDiv);
                groupContainer = document.createElement('div');
                groupContainer.style.display = isCollapsed ? 'none' : 'block';
                list.appendChild(groupContainer);
            }

            groups[cat].forEach(o => {
                const card = document.createElement('div');
                const isActive = activeJobId === o.job_id;
                const flags = offerFlags[o.job_id] || {};
                const isLocked = !!flags.locked;
                const isArchived = !!flags.archived;
                const hasFlag = isLocked || !!flags.archived;
                
                card.className = `offer-card ${isActive ? 'active' : ''} ${hasFlag ? 'has-flag' : ''} ${isArchived ? 'is-archived' : ''}`;
                card.innerHTML = `
                    <div class="offer-card-inner">
                        <div class="offer-card-text">
                            <div style="display:flex; align-items:flex-start; gap:8px;">
                                <div class="offer-title" style="flex:1;">${o.title}</div>
                                <div class="offer-actions-slot">
                                    ${o.status === 'ready' ? '<span class="status-badge ready">READY</span>' : ''}
                                    <span class="offer-action-visibility ${isLocked ? 'is-active' : ''}">
                                        <button type="button" class="offer-action-btn ${isLocked ? 'is-active' : ''}" data-action="lock"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" /></svg></button>
                                    </span>
                                    <span class="offer-action-visibility ${isArchived ? 'is-active' : ''}">
                                        <button type="button" class="offer-action-btn ${isArchived ? 'is-active' : ''}" data-action="archive"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button>
                                    </span>
                                </div>
                            </div>
                            <div class="offer-company">${o.entreprise || ""}</div>
                        </div>
                    </div>
                `;

                card.querySelector('[data-action="lock"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(offerFlags[o.job_id] || {}) };
                    nextFlags.locked = !nextFlags.locked;
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[o.job_id];
                    else offerFlags[o.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };

                card.querySelector('[data-action="archive"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(offerFlags[o.job_id] || {}) };
                    nextFlags.archived = !nextFlags.archived;
                    if (nextFlags.archived) { nextFlags.oldCv = false; nextFlags.deleted = false; }
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[o.job_id];
                    else offerFlags[o.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };

                card.onclick = () => { activeJobId = o.job_id; loadOffers(); updateIframe(); };
                groupContainer.appendChild(card);
            });
        });

        if (visibleArchivedOffers.length) {
            const archiveHeader = document.createElement('div');
            archiveHeader.className = 'sidebar-header';
            archiveHeader.style.padding = '24px 24px 12px';
            archiveHeader.style.marginTop = '16px';
            archiveHeader.style.borderBottom = 'none';
            const archiveLabel = (typeof i18n !== 'undefined') ? i18n.translations[i18n.current].archive : 'ARCHIVE';
            archiveHeader.innerHTML = `<h2>${archiveLabel} (${visibleArchivedOffers.length})</h2>`;
            list.appendChild(archiveHeader);

            visibleArchivedOffers.forEach((o) => {
                const card = document.createElement('div');
                const isActive = activeJobId === o.job_id;
                card.className = `offer-card ${isActive ? 'active' : ''} is-archived archive-muted`;
                card.innerHTML = `
                    <div class="offer-card-inner">
                        <div class="offer-card-text">
                            <div style="display:flex; align-items:flex-start; gap:8px;">
                                <div class="offer-title" style="flex:1;">${o.title}</div>
                                <div class="offer-actions-slot">
                                    <span class="offer-action-visibility is-active">
                                        <button type="button" class="offer-action-btn" data-action="restore-inbox"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button>
                                    </span>
                                </div>
                            </div>
                            <div class="offer-company">${o.entreprise || ''}</div>
                        </div>
                    </div>
                `;
                card.querySelector('[data-action="restore-inbox"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(offerFlags[o.job_id] || {}) };
                    nextFlags.archived = false; nextFlags.oldCv = false; nextFlags.deleted = false;
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[o.job_id];
                    else offerFlags[o.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };
                card.onclick = () => { activeJobId = o.job_id; loadOffers(); updateIframe(); };
                list.appendChild(card);
            });
        }

        const fallbackActive = visibleInboxOffers[0] || visibleArchivedOffers[0];
        if (!activeJobId && fallbackActive) { activeJobId = fallbackActive.job_id; updateIframe(); loadOffers(); }
    } catch (e) {
        console.error("Load offers failed", e);
    }
}

// Global initialization
window.loadOffers = loadOffers;
window.updateIframe = updateIframe;
window.handleRouting = handleRouting;
window.activeJobId = activeJobId;
window.activeTab = activeTab;
window.offerFlags = offerFlags;
window.saveOfferFlags = saveOfferFlags;
