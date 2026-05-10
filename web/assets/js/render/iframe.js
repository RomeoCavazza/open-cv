/**
 * Gère l'état de l'iframe d'aperçu.
 */

export function resetIframeToEmptyState() {
    const iframe = document.getElementById('iframe-doc');
    if (!iframe) return;

    iframe.removeAttribute('srcdoc');
    iframe.src = '/assets/templates/iframe-empty.html';
    
    window.activeInstanceSlug = null;
    window.activeInstanceData = null;
    window.activeResolvedOfferSlug = null;
}

export function renderIframeLoadingState(tab) {
    const iframe = document.getElementById('iframe-doc');
    if (!iframe) return;

    const messages = {
        restitution: "Analyse et Restitution...",
        resume: "Écriture du CV...",
        cover: "Rédaction de la Lettre..."
    };

    const label = messages[tab] || "Génération en cours...";

    const html = `
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {
                    background: #ffffff;
                    color: #1f2937;
                    font-family: 'Inter', -apple-system, sans-serif;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: flex-start;
                    height: 100vh;
                    margin: 0;
                    padding-top: 18vh;
                    overflow: hidden;
                    text-align: center;
                }
                .icon-container {
                    background: #eef2ff;
                    width: 64px;
                    height: 64px;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-bottom: 24px;
                }
                .sparkles {
                    width: 24px;
                    height: 24px;
                    color: #0052ff;
                }
                h2 {
                    font-size: 20px;
                    font-weight: 700;
                    color: #1e293b;
                    margin: 0 0 32px;
                }
                .skeleton-container {
                    width: 100%;
                    max-width: 400px;
                    display: flex;
                    flex-direction: column;
                    gap: 16px;
                    margin: 0 auto;
                }
                .skeleton-bar {
                    height: 14px;
                    background: linear-gradient(90deg, #f3f4f6 25%, #e5e7eb 50%, #f3f4f6 75%);
                    background-size: 200% 100%;
                    animation: loading 1.5s infinite;
                    border-radius: 4px;
                }
                @keyframes loading {
                    0% { background-position: 200% 0; }
                    100% { background-position: -200% 0; }
                }
            </style>
        </head>
        <body>
            <div class="icon-container">
                <svg class="sparkles" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"/><path d="M5 3v4"/><path d="M19 17v4"/><path d="M3 5h4"/><path d="M17 19h4"/></svg>
            </div>
            <h2>${label}</h2>
            <div class="skeleton-container">
                <div class="skeleton-bar" style="width: 100%"></div>
                <div class="skeleton-bar" style="width: 90%"></div>
                <div class="skeleton-bar" style="width: 75%"></div>
            </div>
        </body>
        </html>
    `;

    iframe.src = 'about:blank';
    iframe.srcdoc = html;
}

