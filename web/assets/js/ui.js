import { i18n } from './state.js';
import { clear, el, svg } from './dom.js';

export function updateUIStrings() {
    const t = i18n.translations[i18n.current];
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (t[key]) el.innerText = t[key];
    });
    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
        const key = el.getAttribute('data-i18n-placeholder');
        if (t[key]) el.placeholder = t[key];
    });
    document.querySelectorAll('[data-i18n-title]').forEach(el => {
        const key = el.getAttribute('data-i18n-title');
        if (t[key]) el.title = t[key];
    });
    document.querySelectorAll('[data-i18n-aria-label]').forEach(el => {
        const key = el.getAttribute('data-i18n-aria-label');
        if (t[key]) el.setAttribute('aria-label', t[key]);
    });
    document.querySelectorAll('[data-i18n-value]').forEach(el => {
        const key = el.getAttribute('data-i18n-value');
        if (t[key] && !el.value) el.value = t[key];
    });
}

export function dragHandleMarkup() {
    return el('button', {
        type: 'button',
        className: 'drag-handle',
        dataset: {
            i18nTitle: 'ph_drag_reorder',
            i18nAriaLabel: 'ph_drag_reorder',
        },
        style: 'display:flex; align-items:center; justify-content:center; width:34px; height:34px; border:none; background:transparent; cursor:grab; color:var(--muted-strong); border-radius:8px; padding:0;',
    }, [
        svg('svg', {
            width: '16',
            height: '16',
            viewBox: '0 0 16 16',
            fill: 'currentColor',
            attrs: { 'aria-hidden': 'true' },
        }, [
            [5, 3], [11, 3], [5, 8], [11, 8], [5, 13], [11, 13],
        ].map(([cx, cy]) => svg('circle', { cx, cy, r: '1.4' }))),
    ]);
}

function makeInput(tagName, className, placeholderKey, placeholder, value, extra = {}) {
    return el(tagName, {
        type: tagName === 'textarea' ? undefined : 'text',
        className,
        dataset: { i18nPlaceholder: placeholderKey },
        placeholder,
        value,
        style: extra.style || 'padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;',
    });
}

function makeRemoveButton(onclick) {
    return el('button', {
        type: 'button',
        className: 'muted-remove-btn',
        text: '×',
        onclick,
    });
}

export function stringifyDocument(value) {
    if (value == null) return "";
    try {
        return JSON.stringify(value, null, 2);
    } catch (_) {
        return "";
    }
}

export function readFileAsText(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result || ""));
        reader.onerror = () => reject(new Error('file-read-failed'));
        reader.readAsText(file);
    });
}

export function readFileAsDataUrl(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result || ""));
        reader.onerror = () => reject(new Error('file-read-failed'));
        reader.readAsDataURL(file);
    });
}

export function getDragAfterElement(container, y) {
    const rows = [...container.querySelectorAll('.form-row-exp:not(.dragging-profile-row)')];
    return rows.reduce((closest, child) => {
        const box = child.getBoundingClientRect();
        const offset = y - box.top - box.height / 2;
        if (offset < 0 && offset > closest.offset) {
            return { offset, element: child };
        }
        return closest;
    }, { offset: Number.NEGATIVE_INFINITY, element: null }).element;
}

export function initSortableContainer(container) {
    if (!container || container.dataset.sortableContainerReady === 'true') return;
    container.dataset.sortableContainerReady = 'true';
    container.addEventListener('dragover', (event) => {
        event.preventDefault();
        const dragging = document.querySelector('.dragging-profile-row');
        if (!dragging || dragging.parentElement !== container) return;
        const afterElement = getDragAfterElement(container, event.clientY);
        if (!afterElement) {
            container.appendChild(dragging);
        } else {
            container.insertBefore(dragging, afterElement);
        }
    });
    container.addEventListener('drop', (event) => {
        event.preventDefault();
    });
}

export function setupSortableRow(row) {
    if (row.dataset.sortableReady === 'true') return;
    row.dataset.sortableReady = 'true';
    row.draggable = true;
    row.addEventListener('dragstart', (event) => {
        if (row.dataset.dragReady !== 'true') {
            event.preventDefault();
            return;
        }
        row.classList.add('dragging-profile-row');
        row.style.opacity = '0.55';
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = 'move';
            event.dataTransfer.setData('text/plain', row.dataset.rowId || '');
        }
    });
    row.addEventListener('dragend', () => {
        row.dataset.dragReady = 'false';
        row.classList.remove('dragging-profile-row');
        row.style.opacity = '1';
    });

    const handle = row.querySelector('.drag-handle');
    if (handle) {
        handle.addEventListener('mousedown', () => {
            row.dataset.dragReady = 'true';
        });
        handle.addEventListener('mouseup', () => {
            row.dataset.dragReady = 'false';
        });
        handle.addEventListener('mouseleave', () => {
            row.dataset.dragReady = 'false';
        });
    }
}

export function createExpRow(item = {}) {
    const div = el('div', { className: 'form-row-exp' });
    div.dataset.rowId = `row-${Math.random().toString(36).slice(2, 10)}`;
    div.dataset.dragReady = 'false';
    div.style = "display:grid; grid-template-columns: 34px 1fr 1fr 1fr 40px; gap:10px; margin-bottom:10px; padding:10px; border:1px solid var(--hairline); border-radius:8px; background: var(--canvas); align-items:start;";
    div.appendChild(dragHandleMarkup());

    div.appendChild(makeInput('input', 'exp-role', 'ph_role', 'Titre', item.role || ''));
    div.appendChild(makeInput('input', 'exp-company', 'ph_link', 'Lien', item.company || ''));
    div.appendChild(makeInput('input', 'exp-period', 'ph_period', 'Période', item.period || ''));
    div.appendChild(makeRemoveButton(() => div.remove()));
    div.appendChild(makeInput('textarea', 'exp-desc', 'ph_desc', 'Description', (item.description || []).join('\n'), {
        style: 'grid-column:1 / -1; height:60px; padding:8px; border:1px solid var(--hairline); border-radius:6px; resize:none; font-size:13px;',
    }));
    setupSortableRow(div);
    updateUIStrings();
    return div;
}

export function createEduRow(item = {}) {
    const div = el('div', { className: 'form-row-edu' });
    div.style = "display:grid; grid-template-columns: 1fr 1fr 1fr 40px; gap:10px; margin-bottom:10px;";
    div.appendChild(makeInput('input', 'edu-school', 'ph_school', 'École', item.school || ''));
    div.appendChild(makeInput('input', 'edu-degree', 'ph_degree', 'Diplôme', item.degree || ''));
    div.appendChild(makeInput('input', 'edu-period', 'ph_period', 'Période', item.period || ''));
    div.appendChild(makeRemoveButton(() => div.remove()));
    updateUIStrings();
    return div;
}

export function createLangRow(item = {}) {
    const div = el('div', { className: 'form-row-lang' });
    div.style = "display:grid; grid-template-columns: 1fr 1fr 40px; gap:10px; margin-bottom:10px;";
    div.appendChild(makeInput('input', 'lang-name', 'ph_lang', 'Langue', item.name || ''));
    div.appendChild(makeInput('input', 'lang-level', 'ph_level', 'Niveau', item.level || ''));
    div.appendChild(makeRemoveButton(() => div.remove()));
    updateUIStrings();
    return div;
}

export function normalizeSkillGroup(item = {}) {
    const category = item.category || item.role || "";
    let items = item.items;
    if (!Array.isArray(items)) {
        items = (item.company || "")
            .split(',')
            .map((value) => value.trim())
            .filter(Boolean);
    }
    // Cleanup: remove any trailing '×' or spaces that might have been saved accidentally
    items = items.map(s => typeof s === 'string' ? s.replace(/[ \s×]+$/, '').trim() : s);
    return { category, items };
}

export function createSkillRow(item = { category: "", items: [] }) {
    item = normalizeSkillGroup(item);
    const div = document.createElement('div');
    div.className = 'skill-cat-row';
    div.style = "padding:16px; border:1px solid var(--hairline); border-radius:10px; margin-bottom:12px; background: var(--canvas);";
    const header = document.createElement('div');
    header.style.cssText = 'display:flex; justify-content:space-between; margin-bottom:12px; align-items:center;';

    const categoryInput = document.createElement('input');
    categoryInput.type = 'text';
    categoryInput.className = 'skill-cat-name skill-category-name';
    categoryInput.dataset.i18nPlaceholder = 'ph_cat_name';
    categoryInput.placeholder = 'Nom de catégorie';
    categoryInput.value = item.category;

    const removeButton = document.createElement('button');
    removeButton.type = 'button';
    removeButton.className = 'muted-remove-btn';
    removeButton.textContent = '×';
    removeButton.onclick = () => div.remove();

    header.appendChild(categoryInput);
    header.appendChild(removeButton);

    const pillsContainer = document.createElement('div');
    pillsContainer.className = 'skills-pills-container';
    pillsContainer.style.cssText = 'display:flex; flex-wrap:wrap; gap:8px; margin-bottom:12px;';

    const input = document.createElement('input');
    input.type = 'text';
    input.className = 'skill-input';
    input.dataset.i18nPlaceholder = 'ph_skill_input';
    input.placeholder = 'Ajouter une compétence...';
    input.style.cssText = 'width:100%; padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:12px;';

    div.appendChild(header);
    div.appendChild(pillsContainer);
    div.appendChild(input);

    const renderPills = () => {
        pillsContainer.replaceChildren();
        item.items.forEach((skill, idx) => {
            const pill = document.createElement('div');
            pill.className = 'skill-pill';
            pill.style = "background:var(--soft-blue); color:var(--primary); padding:4px 12px; border-radius:20px; font-size:11px; font-weight:600; display:flex; align-items:center; gap:6px;";
            const skillText = document.createElement('span');
            skillText.className = 'skill-text';
            skillText.textContent = skill;

            const remove = document.createElement('span');
            remove.className = 'skill-pill-remove';
            remove.textContent = '×';
            remove.onclick = () => { item.items.splice(idx, 1); renderPills(); };

            pill.appendChild(skillText);
            pill.appendChild(remove);
            pillsContainer.appendChild(pill);
        });
    };
    input.onkeydown = (e) => {
        if (e.key === 'Enter') {
            const val = input.value.trim();
            if (val && !item.items.includes(val)) { item.items.push(val); input.value = ''; renderPills(); }
        }
    };
    renderPills();
    updateUIStrings();
    return div;
}

export function createAnnexeRow(item = {}) {
    const div = document.createElement('div');
    div.className = 'form-row-annexe';
    div.style = "display:flex; flex-direction:column; gap:0; margin-bottom:12px; border:1px solid var(--hairline); border-radius:10px; background:var(--canvas); overflow:hidden; transition: all 0.2s ease;";

    function createIconSvg(pathDefs, width = '17', height = '17') {
        const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        svg.setAttribute('width', width);
        svg.setAttribute('height', height);
        svg.setAttribute('fill', 'none');
        svg.setAttribute('viewBox', '0 0 24 24');
        svg.setAttribute('stroke-width', '1.5');
        svg.setAttribute('stroke', 'currentColor');
        pathDefs.forEach((def) => {
            const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
            path.setAttribute('stroke-linecap', 'round');
            path.setAttribute('stroke-linejoin', 'round');
            path.setAttribute('d', def);
            svg.appendChild(path);
        });
        return svg;
    }

    // Mapping des données (supporte le format API et le format local)
    const id = item.id || "";
    const label = item.label || item.name || item.filename || 'Nouveau document';
    const contentType = item.content_type || item.type || "";
    const filename = item.filename || item.name || "";
    const dataUrl = item.data_url || "";

    div.dataset.fileId = id;
    div.dataset.fileData = dataUrl;
    div.dataset.fileType = contentType;
    div.dataset.fileName = filename;
    
    const hasFile = !!(id || dataUrl);
    const downloadUrl = id ? `/api/profile/active/annexes/${id}` : dataUrl;

    const header = document.createElement('div');
    header.className = 'annexe-header';
    header.style.cssText = 'display:flex; align-items:center; justify-content:space-between; padding:10px 16px; cursor:default;';

    const left = document.createElement('div');
    left.style.cssText = 'display:flex; align-items:center; gap:12px; flex:1;';

    const iconWrap = document.createElement('div');
    iconWrap.className = 'annexe-icon';
    iconWrap.style.cssText = 'color:var(--primary); display:flex; align-items:center; opacity: 0.5;';
    const fileIcon = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    fileIcon.setAttribute('width', '18');
    fileIcon.setAttribute('height', '18');
    fileIcon.setAttribute('viewBox', '0 0 24 24');
    fileIcon.setAttribute('fill', 'none');
    fileIcon.setAttribute('stroke', 'currentColor');
    fileIcon.setAttribute('stroke-width', '2');
    fileIcon.setAttribute('stroke-linecap', 'round');
    fileIcon.setAttribute('stroke-linejoin', 'round');
    [
        ['path', { d: 'M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z' }],
        ['polyline', { points: '14 2 14 8 20 8' }],
        ['line', { x1: '16', y1: '13', x2: '8', y2: '13' }],
        ['line', { x1: '16', y1: '17', x2: '8', y2: '17' }],
        ['polyline', { points: '10 9 9 9 8 9' }],
    ].forEach(([tag, attrs]) => {
        const node = document.createElementNS('http://www.w3.org/2000/svg', tag);
        Object.entries(attrs).forEach(([key, value]) => node.setAttribute(key, value));
        fileIcon.appendChild(node);
    });
    iconWrap.appendChild(fileIcon);

    const nameInput = document.createElement('input');
    nameInput.type = 'text';
    nameInput.className = 'annexe-name';
    nameInput.value = label;
    nameInput.placeholder = 'Nom du document';
    nameInput.style.cssText = 'background:transparent; border:none; font-size:14px; font-weight:600; color:var(--heading); width:100%; outline:none; padding: 4px 0;';

    const right = document.createElement('div');
    right.style.cssText = 'display:flex; gap:6px; align-items:center;';

    const removeBtn = document.createElement('button');
    removeBtn.type = 'button';
    removeBtn.className = 'annexe-remove-btn';
    removeBtn.textContent = '×';
    removeBtn.style.cssText = 'background:transparent; border:none; color:var(--muted-strong); cursor:pointer; padding:6px; border-radius:50%; font-size:18px; line-height:1; display:flex; align-items:center; justify-content:center; transition: all 0.2s;';

    const fileInput = document.createElement('input');
    fileInput.type = 'file';
    fileInput.className = 'annexe-file-input';
    fileInput.style.display = 'none';

    left.appendChild(iconWrap);
    left.appendChild(nameInput);
    right.appendChild(removeBtn);
    header.appendChild(left);
    header.appendChild(right);
    div.appendChild(header);
    div.appendChild(fileInput);

    iconWrap.style.cursor = 'pointer';
    iconWrap.onclick = () => fileInput.click();
    iconWrap.title = 'Changer le fichier';
    iconWrap.onmouseover = () => { iconWrap.style.color = 'var(--primary)'; };
    iconWrap.onmouseout = () => { 
        iconWrap.style.color = hasFile ? 'var(--primary)' : 'var(--muted-strong)'; 
    };

    // Action d'upload sur l'icône si pas de fichier
    if (!hasFile) {
        iconWrap.style.opacity = '0.5';
    }

    // Hover effect sur removeBtn
    removeBtn.onmouseover = () => { removeBtn.style.color = 'var(--primary)'; };
    removeBtn.onmouseout = () => { removeBtn.style.color = 'var(--muted-strong)'; };
    const openPreview = (e) => {
        if (e) e.stopPropagation();
        if (!hasFile && !div.dataset.fileData) {
            fileInput.click();
            return;
        }
        const modal = document.getElementById('preview-modal');
        const iframe = document.getElementById('preview-modal-iframe');
        const title = document.getElementById('preview-modal-title');
        if (modal && iframe) {
            const url = id ? `/api/profile/active/annexes/${id}` : div.dataset.fileData;
            iframe.src = url;
            if (title) title.innerText = nameInput.value || label;
            modal.style.display = 'flex';
        }
    };

    iconWrap.onclick = openPreview;
    nameInput.style.cursor = 'pointer';
    nameInput.onmouseover = () => { nameInput.style.color = 'var(--primary)'; };
    nameInput.onmouseout = () => { nameInput.style.color = 'var(--heading)'; };
    nameInput.onclick = (e) => {
        if (hasFile || div.dataset.fileData) {
            openPreview(e);
        }
    };
    
    
    removeBtn.onclick = async () => {
        if (div.dataset.fileId) {
            // On marquera pour suppression lors de la sauvegarde du profil, 
            // ou on supprime direct si on veut être radical. 
            // Ici on se contente de retirer du DOM, le dashboard gérera la suite.
            div.dataset.markedForDeletion = "true";
            div.style.display = 'none';
        } else {
            div.remove();
        }
    };
    
    fileInput.onchange = async (e) => {
        const file = e.target.files[0];
        if (!file) return;
        try {
            const dataUrl = await readFileAsDataUrl(file);
            div.dataset.fileData = dataUrl;
            div.dataset.fileType = file.type;
            div.dataset.fileName = file.name;
            
            if (!nameInput.value.trim() || nameInput.value === 'Nouveau document') {
                const rawName = file.name.split('.')[0];
                nameInput.value = rawName.charAt(0).toUpperCase() + rawName.slice(1);
            }
            
            viewBtn.style.display = 'flex';
            viewBtn.style.color = 'var(--primary)';
        } catch (err) {
            console.error("Annexe load failed", err);
        }
    };
    
    updateUIStrings();
    return div;
}

export function renderList(containerId, items, rowCreator) {
    const container = document.getElementById(containerId);
    if (!container) return;
    clear(container);
    items.forEach(item => container.appendChild(rowCreator(item)));
    if (containerId === 'list-experiences' || containerId === 'list-projects') {
        initSortableContainer(container);
    }
}
