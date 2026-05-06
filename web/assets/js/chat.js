// chat.js - Manages the AI chat interaction
import { EVENTS, on } from './modules/events.js';

let pendingAttachments = [];
let currentActiveJobId = null;
let currentLlmProvider = localStorage.getItem('recruitai_llm') || 'ollama';
let currentAbortController = null;

function updateAttachmentsUI() {
    const container = document.getElementById('ai-chat-attachments');
    if (!container) return;

    if (pendingAttachments.length === 0) {
        container.style.display = 'none';
        container.innerHTML = '';
        return;
    }

    container.style.display = 'flex';
    container.style.cssText = 'display:flex; gap:8px; padding:8px; overflow-x:auto; background:#f8fafc; border-bottom:1px solid #e2e8f0;';
    container.innerHTML = '';

    pendingAttachments.forEach((file, index) => {
        const chip = document.createElement('div');
        chip.style.cssText = 'display:flex; align-items:center; gap:6px; background:white; padding:4px 8px; border-radius:16px; border:1px solid #cbd5e1; font-size:11px; white-space:nowrap;';

        const name = document.createElement('span');
        name.textContent = file.name.length > 15 ? file.name.substring(0, 12) + '...' : file.name;

        const remove = document.createElement('button');
        remove.textContent = '×';
        remove.style.cssText = 'border:none; background:none; cursor:pointer; font-weight:bold; color:#64748b; font-size:14px;';
        remove.onclick = () => {
            pendingAttachments.splice(index, 1);
            updateAttachmentsUI();
        };

        chip.appendChild(name);
        chip.appendChild(remove);
        container.appendChild(chip);
    });
}

function appendMessage(role, text, isStreaming = false) {
    const sidebarBody = document.querySelector('.ai-sidebar-body');
    if (!sidebarBody) return null;

    if (isStreaming && role === 'ai') {
        const lastMsg = sidebarBody.lastElementChild;
        if (lastMsg && lastMsg.classList.contains('ai-message') && lastMsg.dataset.streaming === 'true') {
            lastMsg.updateContent(text);
            sidebarBody.scrollTop = sidebarBody.scrollHeight;
            return lastMsg;
        }
    }

    const msgDiv = document.createElement('div');
    msgDiv.className = `chat-message ${role}-message`;
    
    if (role === 'user') {
        msgDiv.style.cssText = `margin-bottom:16px;padding:10px 14px;border-radius:12px;font-size:13.5px;line-height:1.5;max-width:85%;background:#0052ff;color:white;margin-left:auto;box-shadow: 0 2px 4px rgba(0,0,0,0.05);`;
        msgDiv.textContent = text;
    } else {
        msgDiv.className += ' ai-message';
        msgDiv.style.cssText = `margin-bottom:24px;padding:4px 0;font-size:14px;line-height:1.6;color:#1e293b;`;
        if (isStreaming) msgDiv.dataset.streaming = 'true';
        
        const label = document.createElement('div');
        label.textContent = 'RecruitAI';
        label.style.cssText = 'font-size:10px; font-weight:800; color:#94a3b8; text-transform:uppercase; letter-spacing:0.05em; margin-bottom:4px;';
        msgDiv.appendChild(label);
        
        const content = document.createElement('div');
        content.textContent = text;
        msgDiv.appendChild(content);
        
        msgDiv.updateContent = (t) => { content.textContent += t; };
    }

    sidebarBody.appendChild(msgDiv);
    sidebarBody.scrollTop = sidebarBody.scrollHeight;
    return msgDiv;
}

function renderChatHistory(history) {
    const sidebarBody = document.querySelector('.ai-sidebar-body');
    if (!sidebarBody) return;
    sidebarBody.innerHTML = '';

    const entries = Array.isArray(history) ? history : [];
    entries.forEach((entry) => {
        if (!entry || !entry.role) return;
        const role = entry.role === 'assistant' ? 'ai' : 'user';
        appendMessage(role, entry.content || '');
    });

    sidebarBody.scrollTop = sidebarBody.scrollHeight;
}

function setSendButtonBusy(isBusy) {
    const sendBtn = document.querySelector('.ai-send-btn');
    if (!sendBtn) return;
    sendBtn.classList.toggle('is-busy', isBusy);
}

function getActiveOfferSlug() {
    const isAppView = document.getElementById('view-app')?.classList.contains('active');
    if (!isAppView) return null;
    return currentActiveJobId;
}

async function resolveActiveInstance() {
    const offerSlug = getActiveOfferSlug();
    if (!offerSlug) return null;

    const resInstance = await fetch(`/api/offres/${offerSlug}/instance`);
    if (!resInstance.ok) return null;

    return await resInstance.json();
}

async function loadChatHistory() {
    try {
        const instanceData = await resolveActiveInstance();
        let history = [];

        if (instanceData) {
            history = instanceData?.notes?.chat_history || [];
        } else {
            const res = await fetch('/api/profile/active');
            if (res.ok) {
                const profil = await res.json();
                history = profil.notes?.chat_history || [];
            }
        }
        renderChatHistory(history);
    } catch (error) {
        console.warn('Impossible de charger l\'historique du chat', error);
    }
}

async function sendChatMessage() {
    // Si déjà occupé, on agit comme un bouton STOP
    if (currentAbortController) {
        currentAbortController.abort();
        currentAbortController = null;
        setSendButtonBusy(false);
        return;
    }

    const input = document.getElementById('chat-input');
    const message = input.value.trim();
    if (!message) return;

    const offerSlug = getActiveOfferSlug();
    appendMessage('user', message);
    input.value = '';

    const instanceData = offerSlug ? await resolveActiveInstance() : null;
    const instance_id = instanceData?.id || null;

    setSendButtonBusy(true);
    currentAbortController = new AbortController();

    try {
        const response = await fetch('/api/chat/stream', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            signal: currentAbortController.signal,
            body: JSON.stringify({
                message,
                instance_id,
                llm_provider: currentLlmProvider,
                attachments: pendingAttachments.map(a => ({
                    name: a.name,
                    content_type: a.type,
                    data: a.dataUrl
                }))
            })
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let aiMsgElement = null;

        while (true) {
            const { value, done } = await reader.read();
            if (done) break;

            const chunk = decoder.decode(value, { stream: true });
            const lines = chunk.split('\n');
            for (const line of lines) {
                if (line.startsWith('data: ')) {
                    const token = line.slice(6);
                    if (!aiMsgElement) {
                        aiMsgElement = appendMessage('ai', token, true);
                    } else {
                        aiMsgElement.updateContent(token);
                    }
                }
            }
        }
        
        pendingAttachments = [];
        updateAttachmentsUI();

    } catch (e) {
        if (e.name === 'AbortError') {
            console.log("[Chat] Génération arrêtée par l'utilisateur.");
            appendMessage('ai', " (Arrêté)");
        } else {
            console.error("[Chat] Streaming error:", e);
            appendMessage('ai', `Désolé, une erreur est survenue : ${e.message}`);
        }
    } finally {
        currentAbortController = null;
        setSendButtonBusy(false);
    }
}

function initChat() {
    // File attachment handling
    document.addEventListener('change', async (e) => {
        if (e.target.id === 'ai-chat-file-input') {
            const files = Array.from(e.target.files || []);
            for (const file of files) {
                const reader = new FileReader();
                reader.onload = (rev) => {
                    pendingAttachments.push({
                        name: file.name,
                        type: file.type,
                        dataUrl: rev.target.result
                    });
                    updateAttachmentsUI();
                };
                reader.readAsDataURL(file);
            }
            e.target.value = '';
        }
    });

    document.addEventListener('click', (e) => {
        const btn = e.target.closest('#ai-chat-attach-btn');
        if (btn) {
            document.getElementById('ai-chat-file-input').click();
        }
    });

    // Global listener for the Send Button (Event Delegation)
    document.addEventListener('click', (e) => {
        const btn = e.target.closest('#chat-send-btn');
        if (btn) {
            sendChatMessage();
        }
    });

    // Global listener for Enter key in the textarea (Event Delegation)
    document.addEventListener('keydown', (e) => {
        if (e.target.id === 'chat-input' && e.key === 'Enter') {
            if (e.ctrlKey) {
                return;
            } else {
                e.preventDefault();
                // Si on appuie sur Entrée alors que c'est busy, on n'arrête pas (UX choice)
                // On n'envoie que si ce n'est pas busy
                if (!currentAbortController) {
                    sendChatMessage();
                }
            }
        }
    });

    loadChatHistory();
}

// Initial load
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initChat);
} else {
    initChat();
}

// Subscribe to events
on(EVENTS.OFFER_SELECTED, (data) => {
    currentActiveJobId = data.jobId;
    loadChatHistory();
});

on(EVENTS.LLM_PROVIDER_CHANGED, (data) => {
    currentLlmProvider = data.provider;
});

on(EVENTS.INGEST_COMPLETED, () => {
    loadChatHistory();
});

window.loadChatHistory = loadChatHistory;
window.initChat = initChat;
window.sendChatMessage = sendChatMessage;
