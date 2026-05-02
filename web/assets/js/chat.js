// chat.js - Manages the AI chat interaction
async function sendChatMessage() {
    const input = document.getElementById('chat-input');
    const message = input.value.trim();
    if (!message || !window.activeJobId) return;

    // 1. Récupérer l'ID de l'instance via l'API (on a besoin de l'UUID, pas du slug)
    const resOffre = await fetch(`/api/offres/${window.activeJobId}/instance`);
    if (!resOffre.ok) {
        console.error("Instance non trouvée");
        return;
    }
    const instanceData = await resOffre.json();
    const instance_id = instanceData.id;

    // 2. UI - Loading state
    input.disabled = true;
    input.placeholder = "Modification en cours...";
    const sendBtn = document.querySelector('.ai-send-btn');
    if (sendBtn) sendBtn.disabled = true;

    try {
        const resChat = await fetch('/api/chat', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                message,
                instance_id,
                llm_provider: localStorage.getItem('recruitai_llm') || 'ollama'
            })
        });

        if (resChat.ok) {
            input.value = '';
            // Rafraîchir l'iframe pour voir les modifs
            if (typeof window.updateIframe === 'function') window.updateIframe();
        }
    } catch (e) {
        console.error("Chat failed", e);
    } finally {
        input.disabled = false;
        input.placeholder = "Demander des modifications...";
        if (sendBtn) sendBtn.disabled = false;
    }
}

// Attach listener
document.addEventListener('DOMContentLoaded', () => {
    const input = document.getElementById('chat-input');
    const sendBtn = document.querySelector('.ai-send-btn');

    if (input) {
        input.addEventListener('keypress', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendChatMessage();
            }
        });
    }

    if (sendBtn) {
        sendBtn.onclick = sendChatMessage;
    }
});
