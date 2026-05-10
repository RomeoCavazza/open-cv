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
            'stroke-width': '2',
            stroke: 'currentColor',
            attrs: { class: 'offer-action-icon', style: 'width: 14px; height: 14px;' },
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
 * Crée le spinner "Double Span" standard utilisé dans toute l'application.
 */
export function createDoubleSpanSpinner({ size = '14px', color = 'currentColor', className = '' } = {}) {
    const spinner = svg('svg', {
        viewBox: '0 0 24 24',
        fill: 'none',
        stroke: color,
        'stroke-width': '1.5',
        className: `spinner ${className}`,
        style: `width: ${size}; height: ${size}; animation: spin 1s linear infinite;`
    }, [
        svg('path', {
            'stroke-linecap': 'round',
            'stroke-linejoin': 'round',
            d: 'M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99'
        })
    ]);

    // Ensure spin animation exists
    if (!document.getElementById('spinner-style')) {
        const style = document.createElement('style');
        style.id = 'spinner-style';
        style.textContent = '@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }';
        document.head.appendChild(style);
    }

    return spinner;
}

/**
 * Crée une carte d'offre pour la sidebar.
 */
export function createOfferCard(offer, { isActive, isLocked, isArchived, hasFlag, archivedView, status }) {
    const card = document.createElement('div');
    const isGenerating = status && status.toLowerCase() === 'generating';
    card.className = `offer-card ${isActive ? 'active' : ''} ${hasFlag ? 'has-flag' : ''} ${isArchived ? 'is-archived archive-muted' : ''} ${isGenerating ? 'is-generating' : ''}`;
    card.style = `cursor: pointer; transition: all 0.2s;`; // Rely on CSS for padding/background

    const inner = document.createElement('div');
    inner.className = 'offer-card-inner';
    inner.style.width = '100%';

    const text = document.createElement('div');
    text.className = 'offer-card-text';
    text.style.flex = '1';
    text.style.minWidth = '0';

    const titleRow = document.createElement('div');
    titleRow.style.display = 'flex';
    titleRow.style.width = '100%'; // Important pour pousser à droite
    titleRow.style.justifyContent = 'space-between';
    titleRow.style.alignItems = 'flex-start';
    titleRow.style.gap = '12px';

    // --- HEURISTIQUES DE NETTOYAGE AVANCÉES ---
    let displayTitle = offer.title || "Sans titre";
    let displayCompany = offer.entreprise || "";

    // 1. Suppression du bruit Web et redondances
    const noise = ["GESTION DES COOKIES", "SITE WEB", "CAREERS MARKETPLACE", "JOB DETAIL", "ACCUEIL"];
    noise.forEach(n => {
        if (displayTitle.toUpperCase().includes(n)) displayTitle = "Sans titre";
        if (displayCompany.toUpperCase().includes(n)) displayCompany = "";
    });

    // 2. Nettoyage des titres (Parenthèses, F/H, Lieux, Pipes)
    // On retire les (Lieu), (H/F), (F/H), etc.
    displayTitle = displayTitle.replace(/\((.*?)\)/g, "").replace(/\sF\/H/gi, "").replace(/\sH\/F/gi, "").replace(/\sF\s\/\sH/gi, "").trim();
    
    if (displayTitle.includes(" | ")) {
        displayTitle = displayTitle.split(" | ").pop();
    }
    if (displayTitle.includes(" - ") && displayTitle.length > 35) {
        const parts = displayTitle.split(" - ");
        displayTitle = parts[parts.length - 1];
    }
    displayTitle = displayTitle.trim();

    // 3. Détection des inversions
    const contractTypes = ["ALTERNANCE", "APPRENTISSAGE", "STAGE", "INTERNSHIP", "APPRENTICESHIP", "ALTERNANT", "APPRENTI"];
    const isContractOnly = (s) => contractTypes.some(type => s.toUpperCase() === type || s.toUpperCase().includes(type));
    
    if ((displayTitle === "Sans titre" || displayTitle === "") && displayCompany !== "") {
        if (!isContractOnly(displayCompany)) {
            displayTitle = displayCompany;
            displayCompany = "";
        }
    }

    // 4. Nettoyage final entreprise
    if (isContractOnly(displayCompany)) displayCompany = "";
    displayCompany = displayCompany.replace(/\sF\/H/gi, "").replace(/\sH\/F/gi, "").trim();

    const title = document.createElement('div');
    title.className = 'offer-title';
    title.textContent = displayTitle || "Sans titre";

    const actionsSlot = document.createElement('div');
    actionsSlot.className = 'offer-actions-slot';
    if (isLocked || isArchived || isGenerating) actionsSlot.classList.add('has-active'); // Garde le slot visible

    const createActionIcon = (path, active, action, label) => {
        const btn = document.createElement('button');
        btn.dataset.action = action;
        btn.setAttribute('aria-label', label);
        btn.className = 'offer-action-btn'; // Use class instead of inline style
        
        const svgElement = svg('svg', {
            viewBox: '0 0 24 24',
            fill: 'none',
            stroke: 'currentColor',
            'stroke-width': '2',
            className: 'offer-action-icon',
            style: 'width: 14px; height: 14px;'
        }, [
            svg('path', {
                'stroke-linecap': 'round',
                'stroke-linejoin': 'round',
                d: path
            })
        ]);

        if (active) {
            btn.classList.add('is-active');
            svgElement.classList.add('is-active');
        }
        
        btn.appendChild(svgElement);
        return { wrapper: btn };
    };

    if (!isGenerating) {
        if (!archivedView) {
            const lockAction = createActionIcon(
                'M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z',
                isLocked, 'lock', "Verrouiller"
            );
            const archiveAction = createActionIcon(
                'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
                isArchived, 'archive', "Archiver"
            );
            actionsSlot.appendChild(lockAction.wrapper);
            actionsSlot.appendChild(archiveAction.wrapper);
        } else {
            const restoreAction = createActionIcon(
                'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
                false, 'restore-inbox', "Restaurer"
            );
            const deleteAction = createActionIcon(
                'm20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z',
                false, 'send-old', "Supprimer"
            );
            actionsSlot.appendChild(restoreAction.wrapper);
            actionsSlot.appendChild(deleteAction.wrapper);
        }
    }

    if (isGenerating) {
        const spinner = createDoubleSpanSpinner({ size: '14px', color: '#9ca3af', className: 'offer-generating-spinner' });
        spinner.style.marginRight = '4px';
        spinner.title = 'Génération en cours...';
        actionsSlot.insertBefore(spinner, actionsSlot.firstChild);
    }

    const company = document.createElement('div');
    company.className = 'offer-company';
    company.textContent = displayCompany;

    titleRow.appendChild(title);
    titleRow.appendChild(actionsSlot);
    text.appendChild(titleRow);
    text.appendChild(company);
    inner.appendChild(text);
    card.appendChild(inner);

    return card;
}
