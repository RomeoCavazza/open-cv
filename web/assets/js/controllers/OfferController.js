import * as api from '../api.js';
import * as ui from '../ui.js';
import * as offerRender from '../render/offers.js';
import { EVENTS, emit, on } from '../modules/events.js';
import { 
    activeJobId, 
    offerFlags, 
    collapsedOfferCategories, 
    i18n, 
    saveOfferFlags, 
    saveCollapsedCategories,
    setActiveJobId 
} from '../state.js';
import { clear } from '../dom.js';
import * as router from '../router.js';

export class OfferController {
    constructor() {
        this.offersLoadSeq = 0;
        console.log("[OfferController] Initialized");
    }

    async loadOffers() {
        const loadSeq = ++this.offersLoadSeq;
        try {
            const data = await api.fetchOffers();
            const offers = data.entries || [];
            if (loadSeq !== this.offersLoadSeq) return;

            this.renderDashboardApplications(offers);
            this.renderDashboardTreatedOffers(offers);
            this.renderOldOffers(offers);

            const list = document.getElementById('offers-list');
            if (!list) return;
            clear(list);
            const t = i18n.translations[i18n.current];
            const inboxLabel = t.inbox;

            const visibleInboxOffers = offers.filter((offer) => {
                const flags = offerFlags[offer.job_id] || {};
                return !flags.archived && !flags.oldCv && !flags.deleted;
            });

            const visibleArchivedOffers = offers.filter((offer) => {
                const flags = offerFlags[offer.job_id] || {};
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
                const cat = o.category || t.others;
                if (!groups[cat]) groups[cat] = [];
                groups[cat].push(o);
            });

            Object.keys(groups).sort().forEach(cat => {
                const isDefault = cat === t.others || cat.toUpperCase().includes("INBOX");
                const isCollapsed = collapsedOfferCategories.includes(cat);
                let groupContainer = list;

                if (!isDefault) {
                    const catDiv = document.createElement('div');
                    catDiv.className = `sidebar-section-header offer-group-header${isCollapsed ? ' collapsed' : ''}`;

                    const toggle = document.createElement('div');
                    toggle.className = 'offer-group-toggle';

                    const label = document.createElement('span');
                    const translatedCat = t[cat.toLowerCase()] || cat;
                    label.className = 'sidebar-section-title';
                    label.textContent = translatedCat;

                    const count = document.createElement('span');
                    count.className = 'sidebar-section-badge';
                    count.textContent = groups[cat].length;

                    toggle.appendChild(label);
                    toggle.appendChild(count);
                    catDiv.appendChild(toggle);

                    catDiv.onclick = () => this.toggleOfferCategory(cat);
                    list.appendChild(catDiv);
                    groupContainer = document.createElement('div');
                    groupContainer.style.display = isCollapsed ? 'none' : 'block';
                    list.appendChild(groupContainer);
                }

                groups[cat].forEach(o => {
                    const isActive = activeJobId === o.job_id;
                    const flags = offerFlags[o.job_id] || {};
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
                        this.mutateOfferFlags(o.job_id, (nextFlags) => {
                            nextFlags.locked = !nextFlags.locked;
                        });
                    };

                    card.querySelector('[data-action="archive"]').onclick = (event) => {
                        event.stopPropagation();
                        this.mutateOfferFlags(o.job_id, (nextFlags) => {
                            nextFlags.archived = !nextFlags.archived;
                            if (nextFlags.archived) {
                                nextFlags.oldCv = false;
                                nextFlags.deleted = false;
                            }
                        });
                    };

                    card.onclick = () => this.selectOffer(o.job_id);
                    groupContainer.appendChild(card);
                });
            });

            if (visibleArchivedOffers.length) {
                const archiveHeader = document.createElement('div');
                archiveHeader.className = 'sidebar-section-header with-border';

                const archiveTitle = document.createElement('span');
                archiveTitle.className = 'sidebar-section-title';
                archiveTitle.textContent = t.archive;

                const badge = document.createElement('span');
                badge.className = 'sidebar-section-badge';
                badge.textContent = visibleArchivedOffers.length;

                archiveHeader.appendChild(archiveTitle);
                archiveHeader.appendChild(badge);
                list.appendChild(archiveHeader);

                visibleArchivedOffers.forEach((o) => {
                    const isActive = activeJobId === o.job_id;
                    const card = offerRender.createOfferCard(o, {
                        isActive,
                        isLocked: false,
                        isArchived: true,
                        hasFlag: true,
                        archivedView: true,
                    });

                    card.querySelector('[data-action="restore-inbox"]').onclick = (event) => {
                        event.stopPropagation();
                        this.mutateOfferFlags(o.job_id, (nextFlags) => {
                            nextFlags.archived = false;
                            nextFlags.oldCv = false;
                            nextFlags.deleted = false;
                        });
                    };

                    card.querySelector('[data-action="send-old"]').onclick = (event) => {
                        event.stopPropagation();
                        this.mutateOfferFlags(o.job_id, (nextFlags) => {
                            nextFlags.oldCv = true;
                            nextFlags.archived = false;
                            nextFlags.deleted = false;
                        });
                    };

                    card.onclick = () => this.selectOffer(o.job_id);
                    list.appendChild(card);
                });
            }

        } catch (e) {
            console.error('Impossible de charger les offres', e);
        }
    }

    selectOffer(jobId) {
        setActiveJobId(jobId);
        emit(EVENTS.OFFER_SELECTED, { jobId });
    }

    mutateOfferFlags(jobId, mutate) {
        const nextFlags = { ...(offerFlags[jobId] || {}) };
        mutate(nextFlags);
        if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[jobId];
        else offerFlags[jobId] = nextFlags;
        saveOfferFlags();
        this.loadOffers();
    }

    toggleOfferCategory(category) {
        const index = collapsedOfferCategories.indexOf(category);
        if (index >= 0) collapsedOfferCategories.splice(index, 1);
        else collapsedOfferCategories.push(category);
        saveCollapsedCategories();
        this.loadOffers();
    }

    renderDashboardApplications(offers) {
        const panel = document.getElementById('dashboard-applications-panel');
        const list = document.getElementById('dashboard-applications-list');
        if (!panel || !list) return;
        const items = offers.filter((offer) => {
            const flags = offerFlags[offer.job_id] || {};
            return !flags.archived && !flags.oldCv && !flags.deleted;
        });

        list.innerHTML = '';
        if (!items.length) {
            panel.style.display = 'none';
            return;
        }

        panel.style.display = 'block';
        items.forEach((offer) => {
            const flags = offerFlags[offer.job_id] || {};
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
                this.mutateOfferFlags(offer.job_id, (nextFlags) => {
                    nextFlags.archived = !nextFlags.archived;
                    if (nextFlags.archived) {
                        nextFlags.oldCv = false;
                        nextFlags.deleted = false;
                    }
                });
            };

            actions.appendChild(archiveAction.wrapper);
            row.appendChild(text);
            row.appendChild(actions);
            item.appendChild(row);

            item.onclick = () => {
                setActiveJobId(offer.job_id);
                router.switchView('app');
                this.loadOffers();
                emit(EVENTS.UPDATE_IFRAME);
            };
            list.appendChild(item);
        });
    }

    renderDashboardTreatedOffers(offers) {
        const panel = document.getElementById('dashboard-treated-panel');
        const list = document.getElementById('dashboard-treated-list');
        if (!panel || !list) return;
        const items = offers.filter((offer) => {
            const flags = offerFlags[offer.job_id] || {};
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
                        this.mutateOfferFlags(offer.job_id, (nextFlags) => {
                    nextFlags.archived = false;
                    nextFlags.oldCv = false;
                    nextFlags.deleted = false;
                });
            };

            const sendOldAction = offerRender.createOfferActionButton({
                active: true,
                action: 'send-old',
                ariaLabel: 'Archiver définitivement',
                iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
            });
            sendOldAction.button.onclick = (event) => {
                event.stopPropagation();
                        this.mutateOfferFlags(offer.job_id, (nextFlags) => {
                    nextFlags.oldCv = true;
                    nextFlags.archived = false;
                    nextFlags.deleted = false;
                });
            };

            actions.appendChild(restoreAction.wrapper);
            actions.appendChild(sendOldAction.wrapper);
            row.appendChild(text);
            row.appendChild(actions);
            item.appendChild(row);

            item.onclick = () => {
                setActiveJobId(offer.job_id);
                router.switchView('app');
                this.loadOffers();
                emit(EVENTS.UPDATE_IFRAME);
            };
            list.appendChild(item);
        });
    }

    renderOldOffers(offers) {
        const panel = document.getElementById('old-offers-panel');
        const list = document.getElementById('old-offers-list');
        if (!panel || !list) return;
        const oldOffers = offers.filter((offer) => offerFlags[offer.job_id]?.oldCv && !offerFlags[offer.job_id]?.deleted);

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
                        this.mutateOfferFlags(offer.job_id, (nextFlags) => {
                    nextFlags.archived = true;
                    nextFlags.oldCv = false;
                    nextFlags.deleted = false;
                });
            };

            const deleteAction = offerRender.createOfferActionButton({
                active: true,
                action: 'delete',
                ariaLabel: 'Supprimer définitivement',
                iconPath: 'm14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.11 0 0 0-7.5 0',
            });
            deleteAction.button.onclick = () => {
                        this.mutateOfferFlags(offer.job_id, (nextFlags) => {
                    nextFlags.deleted = true;
                    nextFlags.oldCv = false;
                    nextFlags.archived = false;
                });
                if (activeJobId === offer.job_id) setActiveJobId(null);
            };

            actions.appendChild(restoreAction.wrapper);
            actions.appendChild(deleteAction.wrapper);
            row.appendChild(text);
            row.appendChild(actions);
            item.appendChild(row);
            list.appendChild(item);
        });
    }
}
