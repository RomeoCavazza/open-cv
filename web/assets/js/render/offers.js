import { el, svg } from '../dom.js';

/**
 * Crée le bouton d'action (verrou, archive) pour une carte d'offre.
 */
export function createOfferActionButton({ active, action, ariaLabel, iconPath }) {
    const button = el('button', {
        type: 'button',
        className: `offer-action-btn ${active ? 'is-active' : ''}`,
        dataset: { action },
        attrs: { 'aria-label': ariaLabel },
    }, [
        svg('svg', {
            xmlns: 'http://www.w3.org/2000/svg',
            fill: 'none',
            viewBox: '0 0 24 24',
            'stroke-width': '1.5',
            stroke: 'currentColor',
            attrs: { class: 'size-6' },
        }, [
            svg('path', {
                'stroke-linecap': 'round',
                'stroke-linejoin': 'round',
                d: iconPath,
            }),
        ]),
    ]);
    const wrapper = el('span', {
        className: `offer-action-visibility ${active ? 'is-active' : ''}`,
    }, [button]);

    return { wrapper, button };
}

/**
 * Crée une carte d'offre pour la sidebar.
 */
export function createOfferCard(offer, { isActive, isLocked, isArchived, hasFlag, archivedView }) {
    const card = document.createElement('div');
    card.className = `offer-card ${isActive ? 'active' : ''} ${hasFlag ? 'has-flag' : ''} ${isArchived ? 'is-archived archive-muted' : ''}`;
    card.style = `padding: 12px 16px; cursor: pointer; border-radius: 8px; margin: 4px 8px; transition: all 0.2s; background: ${isActive ? 'white' : 'transparent'};`;

    const inner = document.createElement('div');
    inner.className = 'offer-card-inner';

    const text = document.createElement('div');
    text.className = 'offer-card-text';

    const titleRow = document.createElement('div');
    titleRow.style.display = 'flex';
    titleRow.style.alignItems = 'flex-start';
    titleRow.style.gap = '8px';

    const title = document.createElement('div');
    title.className = 'offer-title';
    title.style.flex = '1';
    title.textContent = offer.title;

    const actionsSlot = document.createElement('div');
    actionsSlot.className = 'offer-actions-slot';

    if (!archivedView) {
        const lockAction = createOfferActionButton({
            active: isLocked,
            action: 'lock',
            ariaLabel: "Verrouiller l'offre",
            iconPath: 'M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z',
        });
        const archiveAction = createOfferActionButton({
            active: isArchived,
            action: 'archive',
            ariaLabel: "Archiver l'offre",
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        actionsSlot.appendChild(lockAction.wrapper);
        actionsSlot.appendChild(archiveAction.wrapper);
    } else {
        const restoreAction = createOfferActionButton({
            active: true,
            action: 'restore-inbox',
            ariaLabel: 'Restaurer dans inbox',
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        const sendOldAction = createOfferActionButton({
            active: true,
            action: 'send-old',
            ariaLabel: 'Retirer de la sidebar',
            iconPath: 'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
        });
        actionsSlot.appendChild(restoreAction.wrapper);
        actionsSlot.appendChild(sendOldAction.wrapper);
    }

    const company = document.createElement('div');
    company.className = 'offer-company';
    company.textContent = offer.entreprise || '';

    titleRow.appendChild(title);
    titleRow.appendChild(actionsSlot);
    text.appendChild(titleRow);
    text.appendChild(company);
    inner.appendChild(text);
    card.appendChild(inner);

    return card;
}
