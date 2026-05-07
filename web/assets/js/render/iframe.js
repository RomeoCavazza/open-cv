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
        restitution: "Analyse et Restitution Spark...",
        resume: "Écriture du CV Spark...",
        cover: "Rédaction de la Lettre Spark..."
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
                    justify-content: center;
                    height: 100vh;
                    margin: 0;
                    overflow: hidden;
                }
                .spark-icon {
                    width: 40px;
                    height: 40px;
                    color: #0052ff;
                    margin-bottom: 24px;
                    animation: pulse-spark 2s infinite ease-in-out;
                }
                .skeleton-container {
                    width: 80%;
                    max-width: 500px;
                }
                .skeleton-line {
                    height: 10px;
                    background: linear-gradient(90deg, #f3f4f6 25%, #e5e7eb 50%, #f3f4f6 75%);
                    background-size: 200% 100%;
                    animation: loading 1.5s infinite;
                    border-radius: 5px;
                    margin-bottom: 12px;
                }
                .skeleton-title {
                    height: 20px;
                    width: 60%;
                    margin-bottom: 24px;
                }
                @keyframes loading {
                    0% { background-position: 200% 0; }
                    100% { background-position: -200% 0; }
                }
                @keyframes pulse-spark {
                    0%, 100% { transform: scale(1); opacity: 1; filter: drop-shadow(0 0 0px rgba(0,82,255,0)); }
                    50% { transform: scale(1.1); opacity: 0.8; filter: drop-shadow(0 0 8px rgba(0,82,255,0.4)); }
                }
                .label {
                    margin-top: 16px;
                    font-size: 13px;
                    font-weight: 600;
                    color: #4b5563;
                    letter-spacing: -0.01em;
                }
                .sub-label {
                    margin-top: 4px;
                    font-size: 11px;
                    color: #9ca3af;
                }
            </style>
        </head>
        <body>
            <svg class="spark-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z" />
            </svg>
            <div class="skeleton-container">
                <div class="skeleton-line skeleton-title"></div>
                <div class="skeleton-line" style="width: 100%"></div>
                <div class="skeleton-line" style="width: 90%"></div>
                <div class="skeleton-line" style="width: 40%"></div>
            </div>
            <div class="label">${label}</div>
            <div class="sub-label">RecruitAI Intelligence</div>
        </body>
        </html>
    `;

    iframe.src = 'about:blank';
    iframe.srcdoc = html;
}

