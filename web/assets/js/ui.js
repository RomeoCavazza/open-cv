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
    console.log("[UI] Rendu de l'annexe:", item.label || item.name || item.id);
    const div = document.createElement('div');
    div.className = 'form-row-annexe';
    div.style = "display:flex; flex-direction:column; gap:0; margin-bottom:12px; border:1px solid var(--hairline); border-radius:10px; background:var(--canvas); overflow:hidden; transition: all 0.2s ease;";
    
    const eyeSvg = `<svg width="17" height="17" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" /></svg>`;
    const eyeSlashSvg = `<svg width="17" height="17" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88" /></svg>`;

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

    div.innerHTML = `
        <div class="annexe-header" style="display:flex; align-items:center; justify-content:space-between; padding:10px 16px; cursor:default;">
            <div style="display:flex; align-items:center; gap:12px; flex:1;">
                <div class="annexe-icon" style="color:var(--primary); display:flex; align-items:center; opacity: 0.5;">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>
                </div>
                <input type="text" class="annexe-name" value="${label}" placeholder="Nom du document" style="background:transparent; border:none; font-size:14px; font-weight:600; color:var(--heading); width:100%; outline:none; padding: 4px 0;">
            </div>
            <div style="display:flex; gap:6px; align-items:center;">
                <button type="button" class="annexe-view-btn" style="background:transparent; border:none; color:var(--muted-strong); cursor:pointer; padding:6px; border-radius:50%; display:${hasFile ? 'flex' : 'none'}; align-items:center; justify-content:center; transition: all 0.2s;" title="Aperçu">
                    ${eyeSvg}
                </button>
                <button type="button" class="annexe-upload-btn" style="background:transparent; border:1px dashed var(--hairline); color:var(--muted-strong); cursor:pointer; padding:4px 10px; border-radius:6px; font-size:11px; display:${hasFile ? 'none' : 'block'}; transition: all 0.2s;">
                    Joindre
                </button>
                <button type="button" class="annexe-remove-btn" style="background:transparent; border:none; color:var(--muted-strong); cursor:pointer; padding:6px; border-radius:50%; font-size:18px; line-height:1; display:flex; align-items:center; justify-content:center; transition: all 0.2s;">×</button>
            </div>
        </div>
        <div class="annexe-preview-container" style="display:none; border-top:1px solid var(--hairline); background:white;">
            <iframe class="annexe-preview" style="width:100%; height:500px; border:none; display:block;"></iframe>
        </div>
        <input type="file" class="annexe-file-input" style="display:none;">
    `;
    
    const fileInput = div.querySelector('.annexe-file-input');
    const nameInput = div.querySelector('.annexe-name');
    const preview = div.querySelector('.annexe-preview');
    const previewContainer = div.querySelector('.annexe-preview-container');
    const viewBtn = div.querySelector('.annexe-view-btn');
    const uploadBtn = div.querySelector('.annexe-upload-btn');
    const removeBtn = div.querySelector('.annexe-remove-btn');

    // Hover effects discrets
    [viewBtn, removeBtn].forEach(btn => {
        btn.onmouseover = () => { 
            btn.style.color = 'var(--primary)';
        };
        btn.onmouseout = () => { 
            btn.style.color = 'var(--muted-strong)';
        };
    });
    
    uploadBtn.onmouseover = () => {
        uploadBtn.style.background = 'rgba(0,0,0,0.04)';
        uploadBtn.style.borderColor = 'var(--primary)';
        uploadBtn.style.color = 'var(--primary)';
    };
    uploadBtn.onmouseout = () => {
        uploadBtn.style.background = 'transparent';
        uploadBtn.style.borderColor = 'var(--hairline)';
        uploadBtn.style.color = 'var(--muted-strong)';
    };
    
    if (downloadUrl) {
        preview.src = downloadUrl;
    }
    
    viewBtn.onclick = (e) => {
        e.stopPropagation();
        const isHidden = previewContainer.style.display === 'none';
        previewContainer.style.display = isHidden ? 'block' : 'none';
        viewBtn.innerHTML = isHidden ? eyeSlashSvg : eyeSvg;
        viewBtn.style.color = isHidden ? 'var(--primary)' : 'var(--muted-strong)';
        
        // Lazy load for remote files
        if (isHidden && div.dataset.fileId && !preview.src.includes('/api/')) {
            preview.src = `/api/profile/active/annexes/${div.dataset.fileId}`;
        }
    };
    
    uploadBtn.onclick = () => fileInput.click();
    
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
            
            preview.src = dataUrl;
            viewBtn.style.display = 'flex';
            uploadBtn.style.display = 'none';
            
            previewContainer.style.display = 'block';
            viewBtn.innerHTML = eyeSlashSvg;
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
    container.innerHTML = '';
    items.forEach(item => container.appendChild(rowCreator(item)));
    if (containerId === 'list-experiences' || containerId === 'list-projects') {
        initSortableContainer(container);
    }
}
