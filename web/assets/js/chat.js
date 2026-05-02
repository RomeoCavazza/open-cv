// chat.js - Manages the AI chat interaction
async function appendMessage(role, text) {
    const sidebarBody = document.querySelector('.ai-sidebar-body');
    if (!sidebarBody) return;
    const msgDiv = document.createElement('div');
    msgDiv.className = `chat-message ${role}-message`;
    msgDiv.style.cssText = `margin-bottom:16px;padding:12px 16px;border-radius:12px;font-size:14px;line-height:1.5;max-width:85%;${role === 'user' ? 'background:#0052ff;color:white;margin-left:auto;' : 'background:#f1f5f9;color:#1e293b;border:1px solid #e2e8f0;'}`;
    msgDiv.textContent = text;
    sidebarBody.appendChild(msgDiv);
    sidebarBody.scrollTop = sidebarBody.scrollHeight;
}

async function sendChatMessage() {
    const input = document.getElementById('chat-input');
    const message = input.value.trim();
    if (!message || !window.activeJobId) return;

    appendMessage('user', message);
    input.value = '';

    const resOffre = await fetch(`/api/offres/${window.activeJobId}/instance`);
    if (!resOffre.ok) {
        appendMessage('ai', "Erreur : Instance non trouvée.");
        return;
    }
    const instanceData = await resOffre.json();
    const instance_id = instanceData.id;

    input.disabled = true;
    input.placeholder = "L'IA réfléchit...";
    const sendBtn = document.querySelector('.ai-send-btn');
    if (sendBtn) sendBtn.disabled = true;

    try {
        const resChat = await fetch('/api/chat', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                message,
                instance_id,
                llm_provider: (window.state && window.state.selectedLlmProvider) || localStorage.getItem('recruitai_llm') || 'ollama'
            })
        });

        if (resChat.ok) {
            const result = await resChat.json();
            appendMessage('ai', result.message || "Modifications appliquées !");
            if (typeof window.updateIframe === 'function') window.updateIframe();
        } else {
            appendMessage('ai', "L'IA a rencontré une erreur.");
        }
    } catch (e) {
        appendMessage('ai', "Erreur de connexion.");
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
        input.addEventListener('keydown', (e) => {
            // User request: Enter = send, Ctrl+Enter = newline
            if (e.key === 'Enter') {
                if (e.ctrlKey) {
                    // Let the default behavior (newline) happen
                    return;
                } else {
                    e.preventDefault();
                    sendChatMessage();
                }
            }
        });
    }

    if (sendBtn) {
        sendBtn.onclick = sendChatMessage;
    }
});
