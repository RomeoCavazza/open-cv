import { i18n, aiChatAttachments } from './state.js';
import { clear } from './dom.js';
import { updateUIStrings } from './modules/i18n_ui.js';
import { showToast } from './components/Toast.js';
import { setupSelector } from './components/Selectors.js';
import { 
    createExpRow, 
    createEduRow, 
    createLangRow, 
    createSkillRow, 
    createAnnexeRow,
    initSortableContainer
} from './components/ProfileRows.js';
import { stringifyDocument, readFileAsDataUrl } from './modules/utils.js';

export {
    updateUIStrings,
    showToast,
    setupSelector,
    createExpRow,
    createEduRow,
    createLangRow,
    createSkillRow,
    createAnnexeRow,
    initSortableContainer,
    stringifyDocument,
    readFileAsDataUrl
};

export function renderList(containerId, items, rowCreator) {
    const container = document.getElementById(containerId);
    if (!container) return;
    clear(container);
    items.forEach(item => container.appendChild(rowCreator(item)));
    if (containerId === 'list-experiences' || containerId === 'list-projects') {
        initSortableContainer(container);
    }
}

export function renderAiChatAttachments() {
    const container = document.getElementById('ai-chat-attachments');
    if (!container) return;
    const t = i18n.translations[i18n.current];
    clear(container);

    if (!aiChatAttachments.length) {
        container.style.display = 'none';
        return;
    }

    container.style.display = 'flex';
    aiChatAttachments.forEach((file, index) => {
        const remove = document.createElement('button');
        remove.type = 'button';
        remove.className = 'ai-attachment-remove';
        remove.setAttribute('aria-label', t.attached_files);
        remove.innerText = '×';

        remove.onclick = () => {
            aiChatAttachments.splice(index, 1);
            renderAiChatAttachments();
        };

        const chip = document.createElement('div');
        chip.className = 'ai-attachment-chip';
        const nameSpan = document.createElement('span');
        nameSpan.className = 'ai-attachment-name';
        nameSpan.title = file.name;
        nameSpan.innerText = file.name;

        chip.appendChild(nameSpan);
        chip.appendChild(remove);
        container.appendChild(chip);
    });
}
