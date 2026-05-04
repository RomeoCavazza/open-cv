// chat.js - Manages the AI chat interaction
let pendingAttachments = [];

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
function appendMessage(role, text) {
    const sidebarBody = document.querySelector('.ai-sidebar-body');
    if (!sidebarBody) return;

    const msgDiv = document.createElement('div');
    msgDiv.className = `chat-message ${role}-message`;
    msgDiv.style.cssText = `margin-bottom:16px;padding:12px 16px;border-radius:12px;font-size:14px;line-height:1.5;max-width:85%;${role === 'user' ? 'background:#0052ff;color:white;margin-left:auto;' : 'background:#f1f5f9;color:#1e293b;border:1px solid #e2e8f0;'}`;
    msgDiv.textContent = text;
    sidebarBody.appendChild(msgDiv);
    sidebarBody.scrollTop = sidebarBody.scrollHeight;
}

function renderChatHistory(history) {
    const sidebarBody = document.querySelector('.ai-sidebar-body');
    if (!sidebarBody) return;
    sidebarBody.innerHTML = '';

    const entries = Array.isArray(history) ? history : [];
    if (entries.length === 0) {
    }

    entries.forEach((entry) => {
        if (!entry || !entry.role) return;
        const role = entry.role === 'assistant' ? 'ai' : 'user';
        const text = entry.content || '';
        const msgDiv = document.createElement('div');
        msgDiv.className = `chat-message ${role}-message`;
        msgDiv.style.cssText = `margin-bottom:16px;padding:12px 16px;border-radius:12px;font-size:14px;line-height:1.5;max-width:85%;${role === 'user' ? 'background:#0052ff;color:white;margin-left:auto;' : 'background:#f1f5f9;color:#1e293b;border:1px solid #e2e8f0;'}`;
        msgDiv.textContent = text;
        sidebarBody.appendChild(msgDiv);
    });

    sidebarBody.scrollTop = sidebarBody.scrollHeight;
}

function setSendButtonBusy(isBusy) {
    const sendBtn = document.querySelector('.ai-send-btn');
    if (!sendBtn) return;

    sendBtn.disabled = isBusy;
    sendBtn.classList.toggle('is-busy', isBusy);
}

function getActiveOfferSlug() {
    const isAppView = document.getElementById('view-app')?.classList.contains('active');
    if (!isAppView) return null;
    return window.activeJobId || window.state?.activeJobId || null;
}

async function resolveActiveInstance() {
    const offerSlug = getActiveOfferSlug();
    if (!offerSlug) return null;

    if (window.activeResolvedOfferSlug === offerSlug && window.activeInstanceSlug) {
        if (window.activeInstanceData?.id) {
            return window.activeInstanceData;
        }
        const resInstance = await fetch(`/api/instances/${window.activeInstanceSlug}`);
        if (resInstance.ok) {
            const instanceData = await resInstance.json();
            window.activeInstanceData = instanceData;
            return instanceData;
        }
        return { id: window.activeInstanceSlug, slug: window.activeInstanceSlug };
    }

    const resInstance = await fetch(`/api/offres/${offerSlug}/instance`);
    if (!resInstance.ok) return null;

    const instanceData = await resInstance.json();
    window.activeResolvedOfferSlug = offerSlug;
    window.activeInstanceSlug = instanceData?.slug || null;
    window.activeInstanceData = instanceData;
    return instanceData;
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

async function readChatErrorMessage(response) {
    try {
        const payload = await response.json();
        return payload?.error || payload?.message || `Erreur HTTP ${response.status}`;
    } catch (_) {
        return `Erreur HTTP ${response.status}`;
    }
}

async function sendChatMessage() {
    const input = document.getElementById('chat-input');
    const message = input.value.trim();
    if (!message) return;

    const offerSlug = getActiveOfferSlug();

    appendMessage('user', message);
    input.value = '';

    const instanceData = offerSlug ? await resolveActiveInstance() : null;
    const instance_id = instanceData?.id || null;

    setSendButtonBusy(true);

    try {
        const resChat = await fetch('/api/chat', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                message,
                instance_id,
                llm_provider: window.state?.selectedLlmProvider || localStorage.getItem('recruitai_llm') || 'ollama',
                attachments: pendingAttachments.map(a => ({
                    name: a.name,
                    content_type: a.type,
                    data: a.dataUrl
                }))
            })
        });

        if (resChat.ok) {
            const result = await resChat.json();

            if (result.updated_instance?.slug) {
                window.activeInstanceSlug = result.updated_instance.slug;
                // Defensive: ensure notes is an object if it arrived as a string
                if (typeof result.updated_instance.notes === 'string') {
                    try {
                        result.updated_instance.notes = JSON.parse(result.updated_instance.notes);
                    } catch (e) {
                        console.warn("[Chat] Failed to parse notes string", e);
                    }
                }
                window.activeInstanceData = result.updated_instance;
                window.activeResolvedOfferSlug = offerSlug || window.activeResolvedOfferSlug;
            }

            let persistedHistory = result.updated_instance?.notes?.chat_history;
            if (!Array.isArray(persistedHistory) || persistedHistory.length === 0) {
                const profileResponse = await fetch('/api/profile/active');
                if (profileResponse.ok) {
                    const profile = await profileResponse.json();
                    persistedHistory = profile?.notes?.chat_history;
                } else {
                    console.warn("[Chat] Failed to reload profile history:", profileResponse.status);
                }
            }
            if (Array.isArray(persistedHistory) && persistedHistory.length > 0) {
                pendingAttachments = [];
                updateAttachmentsUI();
                renderChatHistory(persistedHistory);
            } else {
                console.warn("[Chat] No persisted history found in response, falling back to local simulation");
                renderChatHistory([
                    { role: 'user', content: message },
                    { role: 'assistant', content: result.message || "J'ai traité ta demande, mais l'historique de session n'est pas encore synchronisé." },
                ]);
            }
        } else {
            const sidebarBody = document.querySelector('.ai-sidebar-body');
            if (sidebarBody) {
                const errorMessage = await readChatErrorMessage(resChat);
                const msgDiv = document.createElement('div');
                msgDiv.className = 'chat-message ai-message';
                msgDiv.style.cssText = 'margin-bottom:16px;padding:12px 16px;border-radius:12px;font-size:14px;line-height:1.5;max-width:85%;background:#f1f5f9;color:#1e293b;border:1px solid #e2e8f0;';
                msgDiv.textContent = `L'IA a rencontré une erreur : ${errorMessage}`;
                sidebarBody.appendChild(msgDiv);
            }
        }
    } catch (e) {
        const sidebarBody = document.querySelector('.ai-sidebar-body');
        if (sidebarBody) {
            const msgDiv = document.createElement('div');
            msgDiv.className = 'chat-message ai-message';
            msgDiv.style.cssText = 'margin-bottom:16px;padding:12px 16px;border-radius:12px;font-size:14px;line-height:1.5;max-width:85%;background:#f1f5f9;color:#1e293b;border:1px solid #e2e8f0;';
            msgDiv.textContent = `Erreur de connexion : ${e?.message || 'réseau indisponible'}`;
            sidebarBody.appendChild(msgDiv);
        }
    } finally {
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
                sendChatMessage();
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

window.loadChatHistory = loadChatHistory;
window.initChat = initChat;
window.sendChatMessage = sendChatMessage;
