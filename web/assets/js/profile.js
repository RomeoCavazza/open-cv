        let loadedProfileExtras = {};
        let loadedProfileImage = "";
        let loadedApprenticeshipCalendarDocument = null;
        const activeProfileCalendarUrl = '/api/profile/active/calendar';

        function stringifyDocument(value) {
            if (value == null) return "";
            try {
                return JSON.stringify(value, null, 2);
            } catch (_) {
                return "";
            }
        }

        function readFileAsText(file) {
            return new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.onload = () => resolve(String(reader.result || ""));
                reader.onerror = () => reject(new Error('file-read-failed'));
                reader.readAsText(file);
            });
        }

        function readFileAsDataUrl(file) {
            return new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.onload = () => resolve(String(reader.result || ""));
                reader.onerror = () => reject(new Error('file-read-failed'));
                reader.readAsDataURL(file);
            });
        }

        function updateCalendarDocumentUI(documentValue) {
            const preview = document.getElementById('prof-apprenticeship-calendar-preview');
            const legacy = document.getElementById('prof-apprenticeship-calendar-legacy');
            const legacyValue = document.getElementById('prof-apprenticeship-calendar-legacy-value');
            preview.onload = () => {
                if (document.activeElement === preview) preview.blur();
            };

            if (!documentValue) {
                legacy.style.display = 'none';
                preview.src = activeProfileCalendarUrl;
                preview.style.display = 'block';
                if (document.activeElement === preview) preview.blur();
                return;
            }

            if (typeof documentValue === 'string') {
                preview.style.display = 'none';
                legacy.style.display = 'block';
                legacyValue.value = documentValue;
                return;
            }

            if (documentValue.data_url) {
                preview.style.display = 'block';
                preview.src = documentValue.data_url;
                if (document.activeElement === preview) preview.blur();
            }
        }

        async function loadProfile() {
            try {
                const res = await fetch(`/api/profile/active?t=${Date.now()}`, { cache: 'no-store' });
                if (!res.ok) throw new Error();
                const profil = await res.json();
                window.activeProfilId = profil.id || null;
                const content = profil.content;
                loadedProfileExtras = Object.fromEntries(
                    Object.entries(content).filter(([key]) => ![
                        'profile',
                        'apprenticeship',
                        'experiences',
                        'projects',
                        'education',
                        'languages',
                        'skills',
                        'labels'
                    ].includes(key))
                );
                document.getElementById('prof-firstname').value = content.profile?.firstname || "";
                document.getElementById('prof-lastname').value = content.profile?.lastname || "";
                document.getElementById('prof-title').value = content.profile?.title || "";
                document.getElementById('prof-offer-type').value = content.profile?.offer_type || "Alternance";
                document.getElementById('prof-duration').value = content.apprenticeship?.duration || "";
                document.getElementById('prof-rhythm').value = content.apprenticeship?.rhythm || "";
                document.getElementById('prof-pitch').value = content.profile?.pitch || "";
                document.getElementById('prof-location').value = content.profile?.location || "";
                document.getElementById('prof-phone').value = content.profile?.phone || "";
                document.getElementById('prof-email').value = content.profile?.email || "";
                document.getElementById('prof-linkedin').value = content.profile?.linkedin || "";
                document.getElementById('prof-website').value = content.profile?.website || "";
                document.getElementById('prof-github').value = content.profile?.github || "";
                document.getElementById('prof-resume-template').value = stringifyDocument(content.documents?.resume_template || content.resume_template);
                document.getElementById('prof-cover-letter-template').value = stringifyDocument(content.documents?.cover_letter_template || content.cover_letter_template);
                loadedApprenticeshipCalendarDocument = content.documents?.apprenticeship_calendar || null;
                updateCalendarDocumentUI(loadedApprenticeshipCalendarDocument);
                loadedProfileImage = content.profile?.image || "";
                document.getElementById('prof-image-base64').value = loadedProfileImage;
                if (content.profile?.image) {
                    document.getElementById('prof-image-preview').style.backgroundImage = `url(${content.profile.image})`;
                    document.getElementById('preview-placeholder').style.display = 'none';
                } else {
                    document.getElementById('prof-image-preview').style.backgroundImage = '';
                    document.getElementById('preview-placeholder').style.display = 'block';
                }
                renderList('list-experiences', content.experiences || [], createExpRow);
                renderList('list-projects', content.projects || [], createExpRow);
                renderList('list-education', content.education || [], createEduRow);
                renderList('list-languages', content.languages || [], createLangRow);
                renderList('list-skills', content.skills || [], createSkillRow);
                const docs = (content.documents && typeof content.documents === 'object') ? content.documents : {};
                renderList('list-annexes', docs.annexes || [], createAnnexeRow);
                updateUIStrings();
            } catch (e) { console.warn("Profile load failed", e); }
        }

        document.getElementById('btn-clear-resume-template').onclick = () => {
            document.getElementById('prof-resume-template').value = '';
        };
        document.getElementById('btn-clear-cover-letter-template').onclick = () => {
            document.getElementById('prof-cover-letter-template').value = '';
        };
        document.getElementById('btn-clear-apprenticeship-calendar').onclick = () => {
            loadedApprenticeshipCalendarDocument = null;
            updateCalendarDocumentUI(null);
        };

        document.getElementById('prof-image-preview').onclick = () => document.getElementById('prof-image-file').click();
        document.getElementById('ai-chat-attach-btn').onclick = () => document.getElementById('ai-chat-file-input').click();
        document.getElementById('ai-chat-file-input').onchange = (e) => {
            const files = Array.from(e.target.files || []);
            if (!files.length) return;

            files.forEach((file) => {
                const exists = aiChatAttachments.some(
                    (attached) =>
                        attached.name === file.name &&
                        attached.size === file.size &&
                        attached.lastModified === file.lastModified
                );
                if (!exists) aiChatAttachments.push(file);
            });

            e.target.value = '';
            renderAiChatAttachments();
        };

        async function fileToOptimizedDataUrl(file) {
            const dataUrl = await new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.onload = () => resolve(reader.result);
                reader.onerror = () => reject(new Error('image-read-failed'));
                reader.readAsDataURL(file);
            });

            const img = await new Promise((resolve, reject) => {
                const image = new Image();
                image.onload = () => resolve(image);
                image.onerror = () => reject(new Error('image-decode-failed'));
                image.src = dataUrl;
            });

            const maxSize = 512;
            const ratio = Math.min(1, maxSize / Math.max(img.width, img.height));
            const width = Math.max(1, Math.round(img.width * ratio));
            const height = Math.max(1, Math.round(img.height * ratio));

            const canvas = document.createElement('canvas');
            canvas.width = width;
            canvas.height = height;
            const ctx = canvas.getContext('2d');
            if (!ctx) throw new Error('image-canvas-failed');
            ctx.drawImage(img, 0, 0, width, height);

            return canvas.toDataURL('image/jpeg', 0.82);
        }

        document.getElementById('prof-image-file').onchange = (e) => {
            const file = e.target.files[0];
            if (!file) return;
            fileToOptimizedDataUrl(file)
            .then((b64) => {
                loadedProfileImage = b64;
                document.getElementById('prof-image-base64').value = b64;
                document.getElementById('prof-image-preview').style.backgroundImage = `url(${b64})`;
                document.getElementById('preview-placeholder').style.display = 'none';
            })
            .catch((error) => {
                console.error("Profile image processing failed", error);
                alert("Image trop lourde ou illisible.");
            });
        };

        document.getElementById('prof-apprenticeship-calendar-file').onchange = async (e) => {
            const file = e.target.files[0];
            if (!file) return;
            try {
                const dataUrl = await readFileAsDataUrl(file);
                loadedApprenticeshipCalendarDocument = {
                    type: "application/pdf",
                    name: file.name,
                    data_url: dataUrl
                };
                updateCalendarDocumentUI(loadedApprenticeshipCalendarDocument);
            } catch (error) {
                console.error("Apprenticeship calendar load failed", error);
                alert("Le fichier calendrier doit être un PDF lisible.");
            }
        };

        function renderList(containerId, items, rowCreator) {
            const container = document.getElementById(containerId);
            container.innerHTML = '';
            items.forEach(item => container.appendChild(rowCreator(item)));
            if (containerId === 'list-experiences' || containerId === 'list-projects') {
                initSortableContainer(container);
            }
        }

        function dragHandleMarkup() {
            return `
                <button
                    type="button"
                    class="drag-handle"
                    title="Glisser pour réordonner"
                    aria-label="Glisser pour réordonner"
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

        function setupSortableRow(row) {
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

        function getDragAfterElement(container, y) {
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

        function initSortableContainer(container) {
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

        function createExpRow(item = {}) {
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

        function createEduRow(item = {}) {
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

        function createLangRow(item = {}) {
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

        function normalizeSkillGroup(item = {}) {
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

        function createAnnexeRow(item = {}) {
            const div = document.createElement('div');
            div.className = 'doc-field-row form-row-annexe';
            div.style.marginBottom = '24px';
            div.dataset.fileData = item.data_url || "";
            div.dataset.fileType = item.type || "";
            div.dataset.fileName = item.name || "";
            
            div.innerHTML = `
                <div class="doc-field-header" style="margin-bottom:8px;">
                    <div class="doc-field-info">
                        <input type="text" class="annexe-name profile-field-label" value="${item.label || item.name || 'Nouveau document'}" style="background:transparent; border:none; color:var(--heading); padding:0; margin:0; outline:none; width:auto; min-width:200px; cursor:text;" title="Cliquez pour renommer">
                    </div>
                    <div style="display:flex; gap:12px; align-items:center;">
                        <div style="display:flex; gap:4px;">
                            <button type="button" class="muted-remove-btn annexe-remove-btn" style="padding:4px 8px;">×</button>
                        </div>
                    </div>
                </div>
                <div class="annexe-preview-container" style="position:relative; width:100%; height:200px; border:1px solid var(--hairline); border-radius:8px; background:white; overflow:hidden; cursor:pointer;">
                    <iframe class="annexe-preview" style="width:100%; height:100%; border:none; pointer-events:none; ${item.data_url ? '' : 'display:none;'}"></iframe>
                    <div class="annexe-preview-placeholder" style="position:absolute; inset:0; display:${item.data_url ? 'none' : 'flex'}; align-items:center; justify-content:center; color:var(--muted); font-size:12px;">
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



        document.getElementById('add-annexe').onclick = () => document.getElementById('prof-annexe-bulk-file').click();
        
        document.getElementById('prof-annexe-bulk-file').onchange = async (e) => {
            const files = Array.from(e.target.files || []);
            const container = document.getElementById('list-annexes');
            for (const file of files) {
                try {
                    const dataUrl = await readFileAsDataUrl(file);
                    const rawName = file.name.split('.')[0];
                    const capitalized = rawName.charAt(0).toUpperCase() + rawName.slice(1);
                    const item = {
                        label: capitalized,
                        name: file.name,
                        type: file.type,
                        data_url: dataUrl
                    };
                    container.appendChild(createAnnexeRow(item));
                } catch (err) {
                    console.error("Bulk annexe load failed", err);
                }
            }
            e.target.value = '';
        };

        function createSkillRow(item = { category: "", items: [] }) {
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

        document.getElementById('add-exp').onclick = () => {
            const container = document.getElementById('list-experiences');
            container.appendChild(createExpRow());
            initSortableContainer(container);
        };
        document.getElementById('add-project').onclick = () => {
            const container = document.getElementById('list-projects');
            container.appendChild(createExpRow());
            initSortableContainer(container);
        };
        document.getElementById('add-edu').onclick = () => document.getElementById('list-education').appendChild(createEduRow());
        document.getElementById('add-lang').onclick = () => document.getElementById('list-languages').appendChild(createLangRow());
        document.getElementById('add-skill-cat').onclick = () => document.getElementById('list-skills').appendChild(createSkillRow());

        document.getElementById('btn-save-profile').onclick = async () => {
            const btn = document.getElementById('btn-save-profile');
            btn.disabled = true;
            let resumeTemplate;
            let coverLetterTemplate;
            try {
                resumeTemplate = JSON.parse(document.getElementById('prof-resume-template').value || '{}');
                coverLetterTemplate = JSON.parse(document.getElementById('prof-cover-letter-template').value || '{}');
            } catch (error) {
                console.error("Profile document JSON invalid", error);
                alert("Le JSON du modèle CV ou du modèle lettre est invalide.");
                btn.disabled = false;
                return;
            }
            const content = {
                ...loadedProfileExtras,
                profile: {
                    firstname: document.getElementById('prof-firstname').value,
                    lastname: document.getElementById('prof-lastname').value,
                    name: document.getElementById('prof-firstname').value + " " + document.getElementById('prof-lastname').value,
                    image: document.getElementById('prof-image-base64').value || loadedProfileImage || "",
                    title: document.getElementById('prof-title').value,
                    pitch: document.getElementById('prof-pitch').value,
                    location: document.getElementById('prof-location').value,
                    phone: document.getElementById('prof-phone').value,
                    email: document.getElementById('prof-email').value,
                    linkedin: document.getElementById('prof-linkedin').value,
                    website: document.getElementById('prof-website').value,
                    github: document.getElementById('prof-github').value,
                    offer_type: document.getElementById('prof-offer-type').value
                },
                apprenticeship: {
                    duration: document.getElementById('prof-duration').value,
                    rhythm: document.getElementById('prof-rhythm').value,
                    start: "septembre 2026"
                },
                experiences: Array.from(document.querySelectorAll('#list-experiences .form-row-exp')).map(row => ({
                    role: row.querySelector('.exp-role').value,
                    company: row.querySelector('.exp-company').value,
                    period: row.querySelector('.exp-period').value,
                    description: row.querySelector('.exp-desc').value.split('\n').filter(l => l.trim())
                })),
                projects: Array.from(document.querySelectorAll('#list-projects .form-row-exp')).map(row => ({
                    role: row.querySelector('.exp-role').value,
                    company: row.querySelector('.exp-company').value,
                    period: row.querySelector('.exp-period').value,
                    description: row.querySelector('.exp-desc').value.split('\n').filter(l => l.trim())
                })),
                education: Array.from(document.querySelectorAll('.form-row-edu')).map(row => ({
                    school: row.querySelector('.edu-school').value,
                    degree: row.querySelector('.edu-degree').value,
                    period: row.querySelector('.edu-period').value
                })),
                languages: Array.from(document.querySelectorAll('.form-row-lang')).map(row => ({
                    name: row.querySelector('.lang-name').value,
                    level: row.querySelector('.lang-level').value
                })),
                skills: Array.from(document.querySelectorAll('.skill-cat-row')).map(row => ({
                    category: row.querySelector('.skill-cat-name').value,
                    items: Array.from(row.querySelectorAll('.skills-pills-container .skill-pill')).map(p => {
                        const textEl = p.querySelector('.skill-text');
                        return textEl ? textEl.innerText.trim() : p.innerText.replace(' ×', '').trim();
                    })
                })).filter(group => group.category.trim() || group.items.length > 0),
                documents: {
                    resume_template: resumeTemplate,
                    cover_letter_template: coverLetterTemplate,
                    apprenticeship_calendar: loadedApprenticeshipCalendarDocument,
                    annexes: Array.from(document.querySelectorAll('.form-row-annexe')).map(row => ({
                        label: row.querySelector('.annexe-name').value,
                        name: row.dataset.fileName,
                        type: row.dataset.fileType,
                        data_url: row.dataset.fileData
                    })).filter(a => a.label.trim() || a.name)
                },
                labels: { contact: "CONTACT", skills: "COMPÉTENCES", languages: "LANGUES", experiences: "EXPÉRIENCES", projects: "PROJETS", education: "FORMATIONS", download: "Download PDF (A4)" }
            };
            console.log("Saving profile content:", content);
            try {
                const res = await fetch('/api/profile/active', {
                    method: 'PUT',
                    cache: 'no-store',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(content)
                });
                if (!res.ok) {
                    const errorText = await res.text();
                    throw new Error(errorText || `HTTP ${res.status}`);
                }
                alert("Profil sauvegardé !");
                await loadProfile();
            } catch (e) {
                console.error("Profile save failed", e);
                alert("Erreur de sauvegarde du profil.");
            }
            finally { btn.disabled = false; }
        };

        let collapsedOfferCategories = (() => {
            try {
                return JSON.parse(localStorage.getItem('recruitai_collapsed_offer_categories') || '[]');
            } catch (_) {
                return [];
            }
        })();
        let offerFlags = (() => {
            try {
                return JSON.parse(localStorage.getItem('recruitai_offer_flags') || '{}');
            } catch (_) {
                return {};
            }
        })();

        function saveOfferFlags() {
            localStorage.setItem('recruitai_offer_flags', JSON.stringify(offerFlags));
        }

        function renderDashboardApplications(offers) {
            const panel = document.getElementById('dashboard-applications-panel');
            const list = document.getElementById('dashboard-applications-list');
            const items = offers.filter((offer) => {
                const flags = offerFlags[offer.job_id] || {};
                return !flags.archived && !flags.oldCv && !flags.deleted;
            });

            list.innerHTML = '';
            if (!items.length) {
                panel.style.display = 'none';
                return;
            }

            panel.style.display = 'block';
            items.forEach((offer) => {
                const flags = offerFlags[offer.job_id] || {};
                const isArchived = !!flags.archived;
                const item = document.createElement('div');
                item.className = 'old-offer-item';
                item.style.cursor = 'pointer';
                item.innerHTML = `<div class="old-offer-row"><div class="old-offer-text"><div class="old-offer-title">${offer.title}</div><div class="old-offer-company">${offer.entreprise || ''}</div></div><div class="old-offer-actions"><span class="offer-action-visibility"><button type="button" class="offer-action-btn" data-action="edit" aria-label="Éditer l'offre"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" /></svg></button></span><span class="offer-action-visibility ${isArchived ? 'is-active' : ''}"><button type="button" class="offer-action-btn ${isArchived ? 'is-active' : ''}" data-action="archive" aria-label="Archiver l'offre"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></span></div></div>`;
                item.querySelector('[data-action="edit"]').onclick = (event) => {
                    event.stopPropagation();
                    activeJobId = offer.job_id;
                    switchView('app');
                    loadOffers();
                    updateIframe();
                };
                item.querySelector('[data-action="archive"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(offerFlags[offer.job_id] || {}) };
                    nextFlags.archived = !nextFlags.archived;
                    if (nextFlags.archived) {
                        nextFlags.oldCv = false;
                        nextFlags.deleted = false;
                    }
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[offer.job_id];
                    else offerFlags[offer.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };
                item.onclick = () => {
                    activeJobId = offer.job_id;
                    switchView('app');
                    loadOffers();
                    updateIframe();
                };
                list.appendChild(item);
            });
        }

        function renderOldOffers(offers) {
            const panel = document.getElementById('old-offers-panel');
            const list = document.getElementById('old-offers-list');
            const oldOffers = offers.filter((offer) => offerFlags[offer.job_id]?.oldCv && !offerFlags[offer.job_id]?.deleted);

            list.innerHTML = '';
            if (!oldOffers.length) {
                panel.style.display = 'none';
                return;
            }

            panel.style.display = 'block';
            oldOffers.forEach((offer) => {
                const item = document.createElement('div');
                item.className = 'old-offer-item';
                item.innerHTML = `<div class="old-offer-row"><div class="old-offer-text"><div class="old-offer-title">${offer.title}</div><div class="old-offer-company">${offer.entreprise || ''}</div></div><div class="old-offer-actions"><button type="button" class="offer-action-btn" data-action="restore-archive" aria-label="Restaurer dans archive"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button><button type="button" class="offer-action-btn" data-action="delete" aria-label="Supprimer définitivement"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.11 0 0 0-7.5 0" /></svg></button></div></div>`;
                item.querySelector('[data-action="restore-archive"]').onclick = () => {
                    const nextFlags = { ...(offerFlags[offer.job_id] || {}) };
                    nextFlags.archived = true;
                    nextFlags.oldCv = false;
                    nextFlags.deleted = false;
                    offerFlags[offer.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };
                item.querySelector('[data-action="delete"]').onclick = () => {
                    const nextFlags = { ...(offerFlags[offer.job_id] || {}) };
                    nextFlags.deleted = true;
                    nextFlags.oldCv = false;
                    nextFlags.archived = false;
                    offerFlags[offer.job_id] = nextFlags;
                    saveOfferFlags();
                    if (activeJobId === offer.job_id) activeJobId = null;
                    loadOffers();
                };
                list.appendChild(item);
            });
        }

        function renderDashboardTreatedOffers(offers) {
            const panel = document.getElementById('dashboard-treated-panel');
            const list = document.getElementById('dashboard-treated-list');
            const items = offers.filter((offer) => {
                const flags = offerFlags[offer.job_id] || {};
                return flags.archived && !flags.oldCv && !flags.deleted;
            });

            list.innerHTML = '';
            if (!items.length) {
                panel.style.display = 'none';
                return;
            }

            panel.style.display = 'block';
            items.forEach((offer) => {
                const item = document.createElement('div');
                item.className = 'old-offer-item';
                item.style.cursor = 'pointer';
                item.innerHTML = `<div class="old-offer-row"><div class="old-offer-text"><div class="old-offer-title">${offer.title}</div><div class="old-offer-company">${offer.entreprise || ''}</div></div><div class="old-offer-actions"><button type="button" class="offer-action-btn" data-action="restore-inbox" aria-label="Restaurer dans inbox"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0-3-3m3 3 3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button><button type="button" class="offer-action-btn" data-action="send-old" aria-label="Archiver définitivement"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5m6 4.125 2.25 2.25m0 0 2.25 2.25M12 13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z" /></svg></button></div></div>`;
                item.querySelector('[data-action="restore-inbox"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(offerFlags[offer.job_id] || {}) };
                    nextFlags.archived = false;
                    nextFlags.oldCv = false;
                    nextFlags.deleted = false;
                    if (!nextFlags.locked && !nextFlags.archived && !nextFlags.oldCv && !nextFlags.deleted) delete offerFlags[offer.job_id];
                    else offerFlags[offer.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };
                item.querySelector('[data-action="send-old"]').onclick = (event) => {
                    event.stopPropagation();
                    const nextFlags = { ...(offerFlags[offer.job_id] || {}) };
                    nextFlags.oldCv = true;
                    nextFlags.archived = false;
                    nextFlags.deleted = false;
                    offerFlags[offer.job_id] = nextFlags;
                    saveOfferFlags();
                    loadOffers();
                };
                item.onclick = () => {
                    activeJobId = offer.job_id;
                    switchView('app');
                    loadOffers();
                    updateIframe();
                };
                list.appendChild(item);
            });
        }

        window.loadProfile = loadProfile;
        window.renderDashboardApplications = renderDashboardApplications;
        window.renderOldOffers = renderOldOffers;
        window.renderDashboardTreatedOffers = renderDashboardTreatedOffers;
        window.offerFlags = offerFlags;
        window.saveOfferFlags = saveOfferFlags;
