// chat.js - Manages the AI chat interaction
import { EVENTS, emit, on } from './modules/events.js';

let pendingAttachments = [];
let currentActiveJobId = null;
let currentLlmProvider = localStorage.getItem('recruitai_llm') || 'ollama';
let currentAbortController = null;
let currentStatusElement = null;
let currentStatusTimer = null;
let currentStatusPhase = 0;

const STATUS_PHASES = ['Planning', 'Reasoning', 'Generating'];

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

function setStatusPhaseLabel(label) {
    const phaseNode = currentStatusElement?.querySelector('.ai-status-phase');
    if (phaseNode) phaseNode.textContent = label;
}

function startStatusIndicator() {
    stopStatusIndicator();

    const sidebarBody = document.querySelector('.ai-sidebar-body');
    if (!sidebarBody) return;

    currentStatusElement = document.createElement('div');
    currentStatusElement.className = 'chat-message ai-message ai-status-message';
    currentStatusElement.innerHTML = `
        <div class="ai-status-label">RecruitAI</div>
        <div class="ai-status-row">
            <span class="ai-status-phase">${STATUS_PHASES[0]}</span>
            <span class="ai-status-dots" aria-hidden="true">
                <span></span><span></span><span></span>
            </span>
        </div>
    `;

    sidebarBody.appendChild(currentStatusElement);
    sidebarBody.scrollTop = sidebarBody.scrollHeight;

    currentStatusPhase = 0;
    currentStatusTimer = setInterval(() => {
        currentStatusPhase = Math.min(currentStatusPhase + 1, STATUS_PHASES.length - 1);
        setStatusPhaseLabel(STATUS_PHASES[currentStatusPhase]);
    }, 900);
}

function stopStatusIndicator() {
    if (currentStatusTimer) {
        clearInterval(currentStatusTimer);
        currentStatusTimer = null;
    }
    if (currentStatusElement && currentStatusElement.parentNode) {
        currentStatusElement.parentNode.removeChild(currentStatusElement);
    }
    currentStatusElement = null;
    currentStatusPhase = 0;
}

function handleSseEventBlock(block, onToken) {
    let eventType = 'message';
    const dataLines = [];

    block.split('\n').forEach((line) => {
        if (line.startsWith('event:')) {
            eventType = line.slice(6).trim();
        } else if (line.startsWith('data:')) {
            dataLines.push(line.slice(5).replace(/^\s/, ''));
        }
    });

    const payload = dataLines.join('\n');
    if (!payload) return;

    if (eventType === 'error') {
        throw new Error(payload);
    }

    if (eventType === 'status') {
        setStatusPhaseLabel(payload);
        return;
    }

    if (eventType === 'mutation') {
        try {
            const data = JSON.parse(payload);
            // Refresh document iframes with the updated instance
            emit(EVENTS.UPDATE_IFRAME, { source: 'chat', instance: data.instance || data });
            // Visual pop animation on document previews
            document.querySelectorAll('.doc-preview-frame, .preview-iframe').forEach(frame => {
                frame.classList.add('mutation-pop');
                setTimeout(() => frame.classList.remove('mutation-pop'), 600);
            });
        } catch(e) {
            console.warn('[Chat] Mutation parse error', e);
        }
        return;
    }

    if (eventType === 'done') {
        return;
    }

    // Default: treat as token (backward compat with raw "message" events)
    onToken(payload);
}

function drainSseBuffer(buffer, onToken) {
    let nextBuffer = buffer;
    while (true) {
        const separator = nextBuffer.indexOf('\n\n');
        if (separator === -1) break;
        const block = nextBuffer.slice(0, separator).trim();
        nextBuffer = nextBuffer.slice(separator + 2);
        if (!block) continue;
        handleSseEventBlock(block, onToken);
    }
    return nextBuffer;
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
        stopStatusIndicator();
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
    startStatusIndicator();
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

        const reader = response.body?.getReader();
        if (!reader) {
            throw new Error('Le flux de réponse est indisponible.');
        }

        const decoder = new TextDecoder();
        let aiMsgElement = null;
        let sseBuffer = '';

        const onToken = (token) => {
            if (!aiMsgElement) {
                stopStatusIndicator();
                aiMsgElement = appendMessage('ai', token, true);
            } else {
                aiMsgElement.updateContent(token);
            }
        };

        while (true) {
            const { value, done } = await reader.read();
            if (done) break;

            const chunk = decoder.decode(value, { stream: true }).replace(/\r\n/g, '\n');
            sseBuffer += chunk;
            sseBuffer = drainSseBuffer(sseBuffer, onToken);
        }

        const tail = decoder.decode().replace(/\r\n/g, '\n');
        if (tail) {
            sseBuffer += tail;
        }
        if (sseBuffer.trim()) {
            handleSseEventBlock(sseBuffer.trim(), onToken);
        }

        stopStatusIndicator();
        if (offerSlug) {
            emit(EVENTS.UPDATE_IFRAME, { source: 'chat' });
        }

        pendingAttachments = [];
        updateAttachmentsUI();

    } catch (e) {
        stopStatusIndicator();
        if (e.name === 'AbortError') {
            console.log("[Chat] Génération arrêtée par l'utilisateur.");
            appendMessage('ai', " (Arrêté)");
        } else {
            console.error("[Chat] Streaming error:", e);
            appendMessage('ai', `Désolé, une erreur est survenue : ${e.message}`);
        }
    } finally {
        stopStatusIndicator();
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
