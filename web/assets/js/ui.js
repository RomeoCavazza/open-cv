import { i18n } from './state.js';

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
    return `
        <button
            type="button"
            class="drag-handle"
            data-i18n-title="ph_drag_reorder"
            data-i18n-aria-label="ph_drag_reorder"
            style="display:flex; align-items:center; justify-content:center; width:34px; height:34px; border:none; background:transparent; cursor:grab; color:var(--muted-strong); border-radius:8px; padding:0;"
        >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                <circle cx="5" cy="3" r="1.4"></circle>
                <circle cx="11" cy="3" r="1.4"></circle>
                <circle cx="5" cy="8" r="1.4"></circle>
                <circle cx="11" cy="8" r="1.4"></circle>
                <circle cx="5" cy="13" r="1.4"></circle>
                <circle cx="11" cy="13" r="1.4"></circle>
            </svg>
        </button>
    `;
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
    const div = document.createElement('div');
    div.className = 'form-row-exp';
    div.dataset.rowId = `row-${Math.random().toString(36).slice(2, 10)}`;
    div.dataset.dragReady = 'false';
    div.style = "display:grid; grid-template-columns: 34px 1fr 1fr 1fr 40px; gap:10px; margin-bottom:10px; padding:10px; border:1px solid var(--hairline); border-radius:8px; background: var(--canvas); align-items:start;";
    div.innerHTML = `
        ${dragHandleMarkup()}
        <input type="text" data-i18n-placeholder="ph_role" placeholder="Titre" class="exp-role" value="${item.role || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;">
        <input type="text" data-i18n-placeholder="ph_link" placeholder="Lien" class="exp-company" value="${item.company || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;">
        <input type="text" data-i18n-placeholder="ph_period" placeholder="Période" class="exp-period" value="${item.period || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;">
        <button onclick="this.parentElement.remove()" class="muted-remove-btn">×</button>
        <textarea data-i18n-placeholder="ph_desc" placeholder="Description" class="exp-desc" style="grid-column: 1 / -1; height:60px; padding:8px; border:1px solid var(--hairline); border-radius:6px; resize:none; font-size:13px;">${(item.description || []).join('\n')}</textarea>
    `;
    setupSortableRow(div);
    updateUIStrings();
    return div;
}

export function createEduRow(item = {}) {
    const div = document.createElement('div');
    div.className = 'form-row-edu';
    div.style = "display:grid; grid-template-columns: 1fr 1fr 1fr 40px; gap:10px; margin-bottom:10px;";
    div.innerHTML = `
        <input type="text" data-i18n-placeholder="ph_school" placeholder="École" class="edu-school" value="${item.school || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;">
        <input type="text" data-i18n-placeholder="ph_degree" placeholder="Diplôme" class="edu-degree" value="${item.degree || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;">
        <input type="text" data-i18n-placeholder="ph_period" placeholder="Période" class="edu-period" value="${item.period || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:13px;">
        <button onclick="this.parentElement.remove()" class="muted-remove-btn">×</button>
    `;
    updateUIStrings();
    return div;
}

export function createLangRow(item = {}) {
    const div = document.createElement('div');
    div.className = 'form-row-lang';
    div.style = "display:grid; grid-template-columns: 1fr 1fr 40px; gap:10px; margin-bottom:10px;";
    div.innerHTML = `
        <input type="text" data-i18n-placeholder="ph_lang" placeholder="Langue" class="lang-name" value="${item.name || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:8px; font-size:13px;">
        <input type="text" data-i18n-placeholder="ph_level" placeholder="Niveau" class="lang-level" value="${item.level || ""}" style="padding:8px; border:1px solid var(--hairline); border-radius:8px; font-size:13px;">
        <button onclick="this.parentElement.remove()" class="muted-remove-btn">×</button>
    `;
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
    div.innerHTML = `
        <div style="display:flex; justify-content:space-between; margin-bottom:12px; align-items:center;">
            <input type="text" class="skill-cat-name skill-category-name" data-i18n-placeholder="ph_cat_name" placeholder="Nom de catégorie" value="${item.category}">
            <button onclick="this.parentElement.parentElement.remove()" class="muted-remove-btn">×</button>
        </div>
        <div class="skills-pills-container" style="display:flex; flex-wrap:wrap; gap:8px; margin-bottom:12px;"></div>
        <input type="text" class="skill-input" data-i18n-placeholder="ph_skill_input" placeholder="Ajouter une compétence..." style="width:100%; padding:8px; border:1px solid var(--hairline); border-radius:6px; font-size:12px;">
    `;
    const pillsContainer = div.querySelector('.skills-pills-container');
    const input = div.querySelector('.skill-input');
    const renderPills = () => {
        pillsContainer.innerHTML = '';
        item.items.forEach((skill, idx) => {
            const pill = document.createElement('div');
            pill.className = 'skill-pill';
            pill.style = "background:var(--soft-blue); color:var(--primary); padding:4px 12px; border-radius:20px; font-size:11px; font-weight:600; display:flex; align-items:center; gap:6px;";
            pill.innerHTML = `<span class="skill-text">${skill}</span> <span class="skill-pill-remove">×</span>`;
            pill.querySelector('.skill-pill-remove').onclick = () => { item.items.splice(idx, 1); renderPills(); };
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
    div.className = 'doc-field-row form-row-annexe';
    div.style.marginBottom = '24px';
    div.dataset.fileData = item.data_url || "";
    div.dataset.fileType = item.type || "";
    div.dataset.fileName = item.name || "";
    
    div.innerHTML = `
        <div class="doc-field-header" style="margin-bottom:8px;">
            <div class="doc-field-info">
                <input type="text" class="annexe-name profile-field-label" data-i18n-value="ph_annexe_name_default" value="${item.label || item.name || ''}" style="background:transparent; border:none; color:var(--heading); padding:0; margin:0; outline:none; width:auto; min-width:200px; cursor:text;">
            </div>
            <div style="display:flex; gap:12px; align-items:center;">
                <div style="display:flex; gap:4px;">
                    <button type="button" class="muted-remove-btn annexe-remove-btn" style="padding:4px 8px;">×</button>
                </div>
            </div>
        </div>
        <div class="annexe-preview-container" style="position:relative; width:100%; height:200px; border:1px solid var(--hairline); border-radius:8px; background:white; overflow:hidden; cursor:pointer;">
            <iframe class="annexe-preview" style="width:100%; height:100%; border:none; pointer-events:none; ${item.data_url ? '' : 'display:none;'}"></iframe>
            <div class="annexe-preview-placeholder" data-i18n="ph_click_select_file" style="position:absolute; inset:0; display:${item.data_url ? 'none' : 'flex'}; align-items:center; justify-content:center; color:var(--muted); font-size:12px;">
                Cliquez pour sélectionner un fichier
            </div>
        </div>
        <input type="file" class="annexe-file-input" style="display:none;">
    `;
    
    const fileInput = div.querySelector('.annexe-file-input');
    const nameInput = div.querySelector('.annexe-name');
    const preview = div.querySelector('.annexe-preview');
    const previewContainer = div.querySelector('.annexe-preview-container');
    const placeholder = div.querySelector('.annexe-preview-placeholder');
    
    if (item.data_url) {
        preview.src = item.data_url;
    }
    
    const triggerUpload = () => fileInput.click();
    previewContainer.onclick = triggerUpload;
    
    div.querySelector('.annexe-remove-btn').onclick = () => div.remove();
    
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
            preview.src = dataUrl;
            preview.style.display = 'block';
            placeholder.style.display = 'none';
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
    container.innerHTML = '';
    items.forEach(item => container.appendChild(rowCreator(item)));
    if (containerId === 'list-experiences' || containerId === 'list-projects') {
        initSortableContainer(container);
    }
}
