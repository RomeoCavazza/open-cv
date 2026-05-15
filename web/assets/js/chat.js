// chat.js - Manages the AI chat interaction
import { EVENTS, emit, on } from './modules/events.js';
import * as api from './api.js';

let pendingAttachments = [];
let currentActiveJobId = null;
let currentLlmProvider = localStorage.getItem('recruitai_llm') || 'ollama';
let currentAbortController = null;
let currentStatusElement = null;
let currentStatusTimer = null;
let currentStatusPhase = 0;
let snapshotRenderToken = 0;
let restoringVersion = null;
let chatThreads = [];
let currentConversationId = null;
let currentProfileId = null;
let conversationSearchQuery = '';
let hiddenConversationIds = new Set();

const STATUS_PHASES = ['Planning', 'Reasoning', 'Generating'];
let markdownRenderer = null;

function escapeHtml(value) {
    return value
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

function getMarkdownRenderer() {
    if (markdownRenderer) return markdownRenderer;
    if (!window.markdownit) return null;

    markdownRenderer = window.markdownit({
        html: false,
        linkify: true,
        breaks: true,
        typographer: true
    });
    return markdownRenderer;
}

function sanitizeHtml(html) {
    if (!window.DOMPurify) return html;
    return window.DOMPurify.sanitize(html, {
        USE_PROFILES: { html: true }
    });
}

function renderAssistantMarkdown(rawText) {
    if (!rawText) return '';

    const text = rawText
        .replace(/\r\n/g, '\n')
        // Corrige les réponses où les listes numérotées sont collées au paragraphe précédent.
        .replace(/([^\n])(\d+\.\s+\*\*)/g, '$1\n$2');

    const renderer = getMarkdownRenderer();
    if (renderer) {
        return sanitizeHtml(renderer.render(text));
    }

    // Fallback minimal si les libs externes ne chargent pas.
    const escaped = escapeHtml(text).replace(/\n/g, '<br>');
    return `<p>${escaped}</p>`;
}

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

        const content = document.createElement('div');
        content.className = 'ai-message-content';
        let rawContent = text || '';
        content.innerHTML = renderAssistantMarkdown(rawContent);
        msgDiv.appendChild(content);

        msgDiv.updateContent = (t) => {
            rawContent += t;
            content.innerHTML = renderAssistantMarkdown(rawContent);
        };
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

function getConversationTitle(thread) {
    const raw = String(thread?.title || '').trim();
    if (raw) return raw;
    const firstUser = (thread?.messages || []).find((m) => m.role === 'user' && m.content);
    if (!firstUser) return 'Conversation';
    return firstUser.content.split(/\s+/).slice(0, 6).join(' ');
}

function getThreadUpdatedAt(thread) {
    const explicit = thread?.updated_at ? String(thread.updated_at) : '';
    if (explicit) return explicit;
    const messages = Array.isArray(thread?.messages) ? thread.messages : [];
    for (let i = messages.length - 1; i >= 0; i -= 1) {
        const ts = messages[i]?.ts;
        if (ts) return String(ts);
    }
    return '';
}

function getThreadContext(thread) {
    const messages = Array.isArray(thread?.messages) ? thread.messages : [];
    for (let i = messages.length - 1; i >= 0; i -= 1) {
        if (messages[i]?.instance_id) return 'Candidature';
    }
    return 'Global';
}

function formatRelativeTime(value) {
    if (!value) return '';
    const dt = new Date(value);
    if (Number.isNaN(dt.getTime())) return '';

    const diffMs = Date.now() - dt.getTime();
    if (diffMs < 60_000) return 'maintenant';
    const mins = Math.floor(diffMs / 60_000);
    if (mins < 60) return `${mins} min`;
    const hours = Math.floor(mins / 60);
    if (hours < 24) return `${hours} h`;
    const days = Math.floor(hours / 24);
    if (days < 7) return `${days} j`;
    const weeks = Math.floor(days / 7);
    if (weeks < 5) return `${weeks} sem`;
    const months = Math.floor(days / 30);
    return `${months} mois`;
}

function getThreadDateGroup(value) {
    const dt = new Date(value);
    if (Number.isNaN(dt.getTime())) return 'older';
    const now = new Date();
    const dayMs = 24 * 60 * 60 * 1000;
    const startToday = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime();
    const startThread = new Date(dt.getFullYear(), dt.getMonth(), dt.getDate()).getTime();
    const deltaDays = Math.floor((startToday - startThread) / dayMs);

    if (deltaDays <= 0) return 'today';
    if (deltaDays === 1) return 'yesterday';
    if (deltaDays <= 7) return 'week';
    return 'older';
}

function normalizeChatThreads(notes) {
    if (!notes || typeof notes !== 'object') {
        return { threads: [], activeId: null };
    }

    const rawThreads = Array.isArray(notes.chat_threads) ? notes.chat_threads : [];
    const legacy = Array.isArray(notes.chat_history) ? notes.chat_history : [];
    let threads = rawThreads
        .map((t) => ({
            id: String(t.id || ''),
            title: String(t.title || ''),
            updated_at: getThreadUpdatedAt(t),
            context: getThreadContext(t),
            messages: Array.isArray(t.messages) ? t.messages : []
        }))
        .filter((t) => t.id);

    if (threads.length === 0 && legacy.length > 0) {
        threads = [{
            id: 'legacy-default',
            title: 'Conversation',
            updated_at: '',
            context: 'Global',
            messages: legacy
        }];
    }

    threads.sort((a, b) => String(b.updated_at || '').localeCompare(String(a.updated_at || '')));
    const activeId = (notes.active_chat_id && String(notes.active_chat_id)) || threads[0]?.id || null;
    return { threads, activeId };
}

function ensureConversationPanel() {
    const header = document.querySelector('.ai-sidebar-header');
    if (!header) return null;

    let panel = header.querySelector('.ai-conversation-panel');
    if (panel) return panel;

    panel = document.createElement('div');
    panel.className = 'ai-conversation-panel';
    panel.innerHTML = `
        <div class="ai-conversation-head">
            <button type="button" class="ai-chat-toolbar-btn ai-conversation-new" title="Nouveau chat">
                <span>+</span>
            </button>
            <input type="text" class="ai-chat-search" placeholder="Rechercher (Ctrl+K)" aria-label="Recherche conversations" />
        </div>
        <div class="ai-conversation-list"></div>
    `;
    header.prepend(panel);

    panel.querySelector('.ai-conversation-new')?.addEventListener('click', () => {
        createNewConversation();
    });

    panel.querySelector('.ai-chat-search')?.addEventListener('input', (event) => {
        conversationSearchQuery = String(event.target?.value || '');
        renderConversationPanel();
    });

    return panel;
}

function setConversationListHtml(html) {
    const panel = ensureConversationPanel();
    const list = panel?.querySelector('.ai-conversation-list');
    if (!list) return;
    list.innerHTML = html;
}

function createConversationId() {
    if (window.crypto && typeof window.crypto.randomUUID === 'function') {
        return window.crypto.randomUUID();
    }
    return `chat-${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
}

function createNewConversation() {
    currentConversationId = createConversationId();
    hiddenConversationIds.delete(currentConversationId);
    persistHiddenConversations();
    persistSelectedConversation();
    renderConversationPanel();
    renderChatHistory([]);
}

function getConversationStorageKey() {
    if (!currentProfileId) return null;
    return `recruitai_chat_selected_${currentProfileId}`;
}

function getHiddenConversationStorageKey() {
    if (!currentProfileId) return null;
    return `recruitai_chat_hidden_${currentProfileId}`;
}

function restoreSelectedConversationFromStorage() {
    const key = getConversationStorageKey();
    if (!key) return null;
    try {
        return localStorage.getItem(key);
    } catch (_) {
        return null;
    }
}

function persistSelectedConversation() {
    const key = getConversationStorageKey();
    if (!key) return;
    try {
        if (currentConversationId) {
            localStorage.setItem(key, currentConversationId);
        } else {
            localStorage.removeItem(key);
        }
    } catch (_) {
        // ignore localStorage errors
    }
}

function loadHiddenConversationsFromStorage() {
    const key = getHiddenConversationStorageKey();
    hiddenConversationIds = new Set();
    if (!key) return;
    try {
        const raw = localStorage.getItem(key);
        const parsed = JSON.parse(raw || '[]');
        if (Array.isArray(parsed)) {
            hiddenConversationIds = new Set(parsed.map((v) => String(v)));
        }
    } catch (_) {
        hiddenConversationIds = new Set();
    }
}

function persistHiddenConversations() {
    const key = getHiddenConversationStorageKey();
    if (!key) return;
    try {
        localStorage.setItem(key, JSON.stringify(Array.from(hiddenConversationIds)));
    } catch (_) {
        // ignore localStorage errors
    }
}

function renderConversationPanel() {
    const panel = ensureConversationPanel();
    if (!panel) return;

    const searchInput = panel.querySelector('.ai-chat-search');
    if (searchInput && searchInput.value !== conversationSearchQuery) {
        searchInput.value = conversationSearchQuery;
    }

    if (!Array.isArray(chatThreads) || chatThreads.length === 0) {
        setConversationListHtml('<div class="ai-conversation-empty">Aucune conversation.</div>');
        return;
    }

    const query = conversationSearchQuery.trim().toLowerCase();
    let filtered = chatThreads.filter((thread) => !hiddenConversationIds.has(thread.id));
    if (query) {
        filtered = filtered.filter((thread) => {
            const title = getConversationTitle(thread).toLowerCase();
            const context = String(thread.context || '').toLowerCase();
            return title.includes(query) || context.includes(query);
        });
    }

    if (filtered.length === 0) {
        setConversationListHtml('<div class="ai-conversation-empty">Aucun chat trouvé.</div>');
        return;
    }

    const visible = !currentConversationId ? filtered.slice(0, 5) : filtered;
    const groups = { today: [], yesterday: [], week: [], older: [] };
    visible.forEach((thread) => {
        const group = getThreadDateGroup(thread.updated_at);
        if (!groups[group]) groups[group] = [];
        groups[group].push(thread);
    });

    const labels = {
        today: "Aujourd'hui",
        yesterday: 'Hier',
        week: 'Cette semaine',
        older: 'Plus ancien'
    };

    const html = ['today', 'yesterday', 'week', 'older']
        .map((group) => {
            const rows = groups[group];
            if (!rows || rows.length === 0) return '';
            const rowsHtml = rows
                .map((thread) => {
                    const active = thread.id === currentConversationId ? 'active' : '';
                    const title = escapeHtml(getConversationTitle(thread));
                    const context = escapeHtml(thread.context || 'Global');
                    const when = escapeHtml(formatRelativeTime(thread.updated_at));
                    return `
                        <div class="ai-conversation-row ${active}" data-chat-id="${thread.id}">
                            <div class="ai-conversation-main">
                                <div class="ai-conversation-row-title">${title}</div>
                                <div class="ai-conversation-row-sub">${context}</div>
                            </div>
                            <div class="ai-conversation-meta">
                                <span class="ai-conversation-when">${when}</span>
                                <button type="button" class="ai-conversation-delete" data-chat-delete="${thread.id}" title="Masquer">×</button>
                            </div>
                        </div>
                    `;
                })
                .join('');
            return `
                <section class="ai-conversation-group">
                    <div class="ai-conversation-group-title">${labels[group]}</div>
                    ${rowsHtml}
                </section>
            `;
        })
        .join('');

    setConversationListHtml(html);

    document.querySelectorAll('.ai-conversation-row').forEach((row) => {
        row.addEventListener('click', (event) => {
            if (event.target.closest('.ai-conversation-delete')) return;
            const id = row.dataset.chatId;
            if (!id) return;
            currentConversationId = id;
            persistSelectedConversation();
            const thread = chatThreads.find((t) => t.id === id);
            renderConversationPanel();
            renderChatHistory(thread?.messages || []);
        });
    });

    document.querySelectorAll('.ai-conversation-delete').forEach((btn) => {
        btn.addEventListener('click', (event) => {
            event.stopPropagation();
            const id = btn.dataset.chatDelete;
            if (!id) return;
            hiddenConversationIds.add(id);
            persistHiddenConversations();
            if (id === currentConversationId) {
                currentConversationId = null;
                persistSelectedConversation();
                renderChatHistory([]);
            }
            renderConversationPanel();
        });
    });
}

function ensureVersionPanel() {
    const header = document.querySelector('.ai-sidebar-header');
    if (!header) return null;

    let panel = header.querySelector('.ai-version-panel');
    if (panel) return panel;

    panel = document.createElement('div');
    panel.className = 'ai-version-panel';
    panel.innerHTML = `
        <div class="ai-version-panel-head">
            <span class="ai-version-title">Versions</span>
            <button type="button" class="ai-version-refresh" title="Rafraîchir">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="ai-version-refresh-icon">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3" />
                </svg>
            </button>
        </div>
        <div class="ai-version-list"></div>
    `;
    header.appendChild(panel);

    panel.querySelector('.ai-version-refresh')?.addEventListener('click', () => {
        void loadSnapshotHistory();
    });

    return panel;
}

function setVersionPanelVisible(isVisible) {
    const panel = ensureVersionPanel();
    if (!panel) return;
    panel.style.display = isVisible ? '' : 'none';
}

function setVersionListHtml(html) {
    const panel = ensureVersionPanel();
    const list = panel?.querySelector('.ai-version-list');
    if (!list) return;
    list.innerHTML = html;
}

function formatSnapshotDate(value) {
    if (!value) return '';
    const dt = new Date(value);
    if (Number.isNaN(dt.getTime())) return '';
    return dt.toLocaleString(undefined, {
        day: '2-digit',
        month: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    });
}

function wireRestoreButtons(instanceRef) {
    document.querySelectorAll('.ai-version-restore').forEach((btn) => {
        btn.addEventListener('click', async () => {
            const version = Number(btn.dataset.version);
            if (!Number.isFinite(version) || !instanceRef) return;
            if (!confirm(`Restaurer la version ${version} ?`)) return;

            try {
                restoringVersion = version;
                btn.disabled = true;
                const restored = await api.restoreInstanceSnapshot(instanceRef, version);
                emit(EVENTS.UPDATE_IFRAME, { source: 'snapshot_restore', instance: restored });
                appendMessage('ai', `Version ${version} restaurée.`);
                await loadSnapshotHistory(restored);
                if (currentActiveJobId) {
                    loadChatHistory(restored).catch(() => {});
                }
            } catch (error) {
                console.error('[Chat] Restore snapshot error', error);
                appendMessage('ai', `Impossible de restaurer la version ${version}: ${error.message}`);
            } finally {
                restoringVersion = null;
            }
        });
    });
}

async function loadSnapshotHistory(instanceOverride = null) {
    const offerSlug = getActiveOfferSlug();
    if (!offerSlug) {
        setVersionPanelVisible(false);
        return;
    }

    const token = ++snapshotRenderToken;
    setVersionPanelVisible(false);

    try {
        const instanceData = instanceOverride || await resolveActiveInstance();
        if (!instanceData) {
            if (token !== snapshotRenderToken) return;
            setVersionPanelVisible(false);
            return;
        }

        const instanceRef = instanceData.id || instanceData.slug;
        const snapshots = await api.fetchInstanceSnapshots(instanceRef);
        if (token !== snapshotRenderToken) return;

        if (!Array.isArray(snapshots) || snapshots.length === 0) {
            setVersionPanelVisible(false);
            return;
        }

        const items = snapshots
            .slice(0, 20)
            .map((snap) => {
                const msg = (snap.trigger_message || 'Sans message').replace(/</g, '&lt;');
                const when = formatSnapshotDate(snap.created_at);
                const isRestoring = restoringVersion === snap.version;
                return `
                    <div class="ai-version-item">
                        <div class="ai-version-meta">
                            <span class="ai-version-tag">V${snap.version}</span>
                            <span class="ai-version-when">${when}</span>
                        </div>
                        <div class="ai-version-msg">${msg}</div>
                        <button type="button" class="ai-version-restore" data-version="${snap.version}" ${isRestoring ? 'disabled' : ''}>
                            Restaurer
                        </button>
                    </div>
                `;
            })
            .join('');

        setVersionPanelVisible(true);
        setVersionListHtml(items);
        wireRestoreButtons(instanceRef);
    } catch (error) {
        if (token !== snapshotRenderToken) return;
        console.error('[Chat] Snapshot load error', error);
        setVersionPanelVisible(false);
    }
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
            // SSE spec: on retire au plus UN espace optionnel après "data:".
            let data = line.slice(5);
            if (data.startsWith(' ')) data = data.slice(1);
            dataLines.push(data);
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
            const instancePayload = data.instance || data;
            // Refresh document iframes with the updated instance
            emit(EVENTS.UPDATE_IFRAME, { source: 'chat', instance: instancePayload });
            void loadSnapshotHistory(instancePayload);
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
        const block = nextBuffer.slice(0, separator);
        nextBuffer = nextBuffer.slice(separator + 2);
        if (!block || !block.replace(/\n/g, '').trim()) continue;
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

async function loadChatHistory(instanceOverride = null) {
    try {
        const instanceData = instanceOverride || await resolveActiveInstance();
        const res = await fetch('/api/profile/active');
        if (!res.ok) {
            renderChatHistory([]);
            return instanceData;
        }
        const profil = await res.json();
        currentProfileId = profil?.id || null;
        const normalized = normalizeChatThreads(profil.notes || {});
        chatThreads = normalized.threads;
        loadHiddenConversationsFromStorage();

        const persistedId = restoreSelectedConversationFromStorage();
        if (!currentConversationId && persistedId) {
            currentConversationId = persistedId;
        } else if (!currentConversationId && normalized.activeId) {
            currentConversationId = normalized.activeId;
        }
        if (currentConversationId && hiddenConversationIds.has(currentConversationId)) {
            currentConversationId = null;
            persistSelectedConversation();
        }
        if (currentConversationId && !chatThreads.some((t) => t.id === currentConversationId)) {
            currentConversationId = null;
            persistSelectedConversation();
        }

        renderConversationPanel();
        const activeThread = chatThreads.find((t) => t.id === currentConversationId);
        renderChatHistory(activeThread?.messages || []);
        return instanceData;
    } catch (error) {
        console.warn('Impossible de charger l\'historique du chat', error);
        return null;
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
    if (!currentConversationId) {
        currentConversationId = createConversationId();
        persistSelectedConversation();
        renderConversationPanel();
    }

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
                conversation_id: currentConversationId,
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
            handleSseEventBlock(sseBuffer, onToken);
        }

        stopStatusIndicator();
        if (offerSlug) {
            emit(EVENTS.UPDATE_IFRAME, { source: 'chat' });
            void loadSnapshotHistory();
        }
        await loadChatHistory();

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
        const withCommand = e.metaKey || e.ctrlKey;
        if (withCommand && e.key.toLowerCase() === 'k') {
            e.preventDefault();
            const input = document.querySelector('.ai-chat-search');
            if (input) {
                input.focus();
                input.select();
            }
            return;
        }
        if (withCommand && e.key.toLowerCase() === 'n') {
            e.preventDefault();
            createNewConversation();
            return;
        }

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

    loadChatHistory().then((instanceData) => {
        void loadSnapshotHistory(instanceData);
    });
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
    loadChatHistory().then((instanceData) => {
        void loadSnapshotHistory(instanceData);
    });
});

on(EVENTS.LLM_PROVIDER_CHANGED, (data) => {
    currentLlmProvider = data.provider;
});

on(EVENTS.INGEST_COMPLETED, () => {
    loadChatHistory().then((instanceData) => {
        void loadSnapshotHistory(instanceData);
    });
});

window.loadChatHistory = loadChatHistory;
window.initChat = initChat;
window.sendChatMessage = sendChatMessage;
