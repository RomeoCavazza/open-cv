import * as state from './state.js';

/**
 * Gestion centralisée du routage et des vues.
 */

let views = {};
let callbacks = {
    onLoadOffers: () => {},
    onResetIframe: () => {},
    onLoadChatHistory: () => {}
};

export function initRouter(config) {
    views = config.views;
    callbacks = { ...callbacks, ...config.callbacks };
    window.addEventListener('popstate', handleRouting);
}

export function switchView(viewName) {
    if (!views[viewName]) return;

    Object.values(views).forEach(v => {
        if (v) {
            v.classList.remove('active');
            v.scrollTop = 0;
        }
    });

    if (views[viewName]) {
        views[viewName].classList.add('active');
        views[viewName].scrollTop = 0;
    }

    document.querySelectorAll('.nav-link').forEach(l => l.classList.remove('active'));
    
    if (viewName === 'ingest') {
        document.getElementById('nav-dashboard').classList.add('active');
        callbacks.onLoadOffers();
    }
    if (viewName === 'app') {
        document.getElementById('nav-app').classList.add('active');
        callbacks.onLoadOffers();
        if (!state.activeJobId) callbacks.onResetIframe();
    }
    if (viewName === 'profile') {
        document.getElementById('nav-profile').classList.add('active');
    }

    updatePath();
    callbacks.onLoadChatHistory();
}

export function updatePath() {
    let path = '/';
    if (views.app && views.app.classList.contains('active')) {
        path = '/applications';
        if (state.activeJobId) {
            path += `/${state.activeJobId}`;
            if (state.activeTab) path += `/${state.activeTab}`;
        }
    } else if (views.profile && views.profile.classList.contains('active')) {
        path = '/profil';
    }
    
    if (window.location.pathname !== path) {
        history.pushState(null, null, path);
    }
}

export async function handleRouting() {
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
            
            await callbacks.onLoadOffers();
            
            if (state.activeTab) {
                const tab = document.querySelector(`.tab[data-target="${state.activeTab}"]`);
                if (tab) tab.click();
            }
        }
    } else if (parts[0] === 'profil') {
        switchView('profile');
    }
}
