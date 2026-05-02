// --- I18N SYSTEM ---
        const i18n = {
            current: 'fr',
            translations: {
                fr: {
                    new_application: "Nouvelle candidature",
                    job_description: "DESCRIPTION DU POSTE", placeholder_job_input: "Collez l'URL ou le texte ici...",
                    deliverables: "LIVRABLES", context: "CONTEXTE", generate_app: "Générer la Candidature",
                    edit_profile: "Éditer le Profil", save_changes: "Enregistrer les modifications",
                    identity_photo: "Identité & Photo", click_upload: "Cliquez pour upload", load_document: "Charger",
                    templates_section: "Templates", templates_help: "Importez vos modèles JSON, puis éditez-les si besoin.",
                    attachments_section: "Pièces jointes", attachments_help: "Ajoutez vos documents de référence et remplacez-les à tout moment.",
                    import_json: "Importer JSON", add_file: "Ajouter un fichier", replace: "Remplacer", add_document: "+ Ajouter un document",
                    attached_files: "Fichiers joints",
                    no_file: "Aucun fichier", json_loaded: "JSON chargé", json_edited: "JSON édité", legacy_text_value: "Valeur texte (obsolète)",
                    lastname: "Nom", firstname: "Prénom", application_context: "Contexte de Candidature",
                    target_job: "Titre visé", offer_type: "Type d'offre", duration: "Durée souhaitée",
                    rhythm: "Rythme", bio: "Bio", contact_networks: "Contact & Réseaux",
                    city: "Ville", phone: "Téléphone", email: "Email", website: "Site Personnel",
                    experiences: "Expériences Professionnelles", projects: "Projets Perso", education: "Formations",
                    skills: "Compétences", languages: "Langues", add: "+ Ajouter",
                    documents_annexes: "Documents", resume_template: "Modèle CV", cover_letter_template: "Modèle lettre", apprenticeship_calendar: "Calendrier d'alternance",
                    annexes: "Annexes supplémentaires", ph_annexe_name: "Nom du document",
                    restitution: "Restitution", resume: "CV", cover_letter: "Lettre de motivation", download: "Télécharger PDF",
                    ph_role: "Titre", ph_link: "Lien", ph_period: "Période", ph_desc: "Description (bullet points)",
                    ph_school: "École", ph_degree: "Diplôme", ph_lang: "Langue", ph_level: "Niveau", ph_skill_input: "Ajouter une compétence...",
                    ph_cat_name: "Nom de catégorie", inbox: "INBOX", archive: "ARCHIVE", old_offer: "ANCIENNE OFFRE", old_applications: "Anciennes candidatures", applications_in_progress: "Candidatures en cours", applications_treated: "Candidatures traitées", applications_list: "Candidatures",
                    nav_dashboard: "Dashboard", nav_applications: "Applications", nav_profile: "Profil",
                    ai_prompt_placeholder: "Demander des modifications"
                },
                en: {
                    new_application: "New Application",
                    job_description: "JOB DESCRIPTION", placeholder_job_input: "Paste URLs or text here...",
                    deliverables: "DELIVERABLES", context: "CONTEXT", generate_app: "Generate Application",
                    edit_profile: "Edit Profile", save_changes: "Save Changes",
                    identity_photo: "Identity & Photo", click_upload: "Click to upload", load_document: "Load",
                    templates_section: "Templates", templates_help: "Import your JSON templates, then edit them if needed.",
                    attachments_section: "Attachments", attachments_help: "Add your reference documents and replace them anytime.",
                    import_json: "Import JSON", add_file: "Add file", replace: "Replace", add_document: "+ Add document",
                    attached_files: "Attached files",
                    no_file: "No file", json_loaded: "Loaded JSON", json_edited: "Edited JSON", legacy_text_value: "Legacy text value",
                    lastname: "Last Name", firstname: "First Name", application_context: "Application Context",
                    target_job: "Target Job Title", offer_type: "Offer Type", duration: "Desired Duration",
                    rhythm: "Rhythm", bio: "Bio", contact_networks: "Contact & Networks",
                    city: "City", phone: "Phone", email: "Email", website: "Personal Website",
                    experiences: "Work Experience", projects: "Personal Projects", education: "Education",
                    skills: "Skills", languages: "Languages", add: "+ Add",
                    documents_annexes: "Documents", resume_template: "Resume template", cover_letter_template: "Cover letter template", apprenticeship_calendar: "Apprenticeship calendar",
                    annexes: "Supplementary Annexes", ph_annexe_name: "Document Name",
                    restitution: "Restitution", resume: "Resume", cover_letter: "Cover Letter", download: "Download PDF",
                    ph_role: "Title", ph_link: "Link", ph_period: "Period", ph_desc: "Description (bullet points)",
                    ph_school: "School", ph_degree: "Degree", ph_lang: "Language", ph_level: "Level", ph_skill_input: "Add skill... (Enter)",
                    ph_cat_name: "Category Name", inbox: "INBOX", archive: "ARCHIVE", old_offer: "OLD OFFERS", old_applications: "Old applications", applications_in_progress: "Applications in progress", applications_treated: "Processed applications", applications_list: "Applications",
                    nav_dashboard: "Dashboard", nav_applications: "Applications", nav_profile: "Profile",
                    ai_prompt_placeholder: "Request changes"
                }
            }
        };

        function updateUIStrings() {
            const t = i18n.translations[i18n.current];
            document.querySelectorAll('[data-i18n]').forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (t[key]) el.innerText = t[key];
            });
            document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
                const key = el.getAttribute('data-i18n-placeholder');
                if (t[key]) el.placeholder = t[key];
            });
        }

        document.querySelectorAll('.lang-toggle').forEach(sel => {
            sel.onchange = (e) => {
                i18n.current = e.target.value;
                document.querySelectorAll('.lang-toggle').forEach(s => s.value = i18n.current);
                updateUIStrings();
                renderAiChatAttachments();
                loadOffers();
            };
        });

        const views = { ingest: document.getElementById('view-ingest'), app: document.getElementById('view-app'), profile: document.getElementById('view-profile') };
        let activeJobId = null;
        let activeTab = 'restitution';
        let aiChatAttachments = [];

        function renderAiChatAttachments() {
            const container = document.getElementById('ai-chat-attachments');
            const t = i18n.translations[i18n.current];
            container.innerHTML = '';

            if (!aiChatAttachments.length) {
                container.style.display = 'none';
                return;
            }

            container.style.display = 'flex';
            aiChatAttachments.forEach((file, index) => {
                const chip = document.createElement('div');
                chip.className = 'ai-attachment-chip';
                chip.innerHTML = `
                    <span class="ai-attachment-name" title="${file.name}">${file.name}</span>
                    <button type="button" class="ai-attachment-remove" aria-label="${t.attached_files}">×</button>
                `;
                chip.querySelector('.ai-attachment-remove').onclick = () => {
                    aiChatAttachments.splice(index, 1);
                    renderAiChatAttachments();
                };
                container.appendChild(chip);
            });
        }

        function switchView(viewName) {
            Object.values(views).forEach(v => {
                v.classList.remove('active');
                v.scrollTop = 0;
            });
            views[viewName].classList.add('active');
            views[viewName].scrollTop = 0;
            document.querySelectorAll('.nav-link').forEach(l => l.classList.remove('active'));
            if (viewName === 'ingest') document.getElementById('nav-dashboard').classList.add('active');
            if (viewName === 'app') { document.getElementById('nav-app').classList.add('active'); loadOffers(); }
            if (viewName === 'profile') document.getElementById('nav-profile').classList.add('active');
            updatePath();
        }

        function updatePath() {
            let path = '/';
            if (views.app.classList.contains('active')) {
                path = '/applications';
                if (activeJobId) {
                    path += `/${activeJobId}`;
                    if (activeTab) path += `/${activeTab}`;
                }
            } else if (views.profile.classList.contains('active')) {
                path = '/profil';
            }
            if (window.location.pathname !== path) {
                history.pushState(null, null, path);
            }
        }

        async function handleRouting() {
            const path = window.location.pathname;
            if (!path || path === '/') {
                switchView('ingest');
                return;
            }

            const parts = path.split('/').filter(Boolean); // applications, slug, tab
            if (parts[0] === 'applications') {
                switchView('app');
                if (parts[1]) {
                    activeJobId = parts[1];
                    if (parts[2]) activeTab = parts[2];
                    
                    await loadOffers();
                    
                    if (activeTab) {
                        const tab = document.querySelector(`.tab[data-target="${activeTab}"]`);
                        if (tab) tab.click();
                    }
                }
            } else if (parts[0] === 'profil') {
                switchView('profile');
            }
        }
        window.addEventListener('popstate', handleRouting);

        function readSavedDeliverables() {
            try {
                const saved = JSON.parse(localStorage.getItem('recruitai_delivs') || '{}');
                return {
                    restitution: saved.restitution !== false,
                    resume: saved.resume !== false,
                    cover: saved.cover !== false
                };
            } catch (error) {
                console.warn('Invalid saved deliverables, fallback to defaults', error);
                return { restitution: true, resume: true, cover: true };
            }
        }

        let selectedLlmProvider = localStorage.getItem('recruitai_llm') || 'ollama';
        
        function syncLlmSelectors(provider) {
            selectedLlmProvider = provider;
            localStorage.setItem('recruitai_llm', provider);
            document.querySelectorAll('.llm-pill[data-provider]').forEach(p => {
                if (p.dataset.provider === provider) p.classList.add('active');
                else p.classList.remove('active');
            });
        }

        document.querySelectorAll('.llm-pill[data-provider]').forEach(pill => {
            pill.onclick = () => syncLlmSelectors(pill.dataset.provider);
        });

        // Load persisted deliverables
        const savedDelivs = readSavedDeliverables();
        document.querySelectorAll('#deliv-selector-ingest .llm-pill').forEach(pill => {
            const deliv = pill.dataset.deliv;
            pill.classList.toggle('active', !!savedDelivs[deliv]);
            pill.onclick = () => {
                pill.classList.toggle('active');
                const current = {};
                document.querySelectorAll('#deliv-selector-ingest .llm-pill').forEach(p => {
                    current[p.dataset.deliv] = p.classList.contains('active');
                });
                localStorage.setItem('recruitai_delivs', JSON.stringify(current));
            };
        });

        syncLlmSelectors(selectedLlmProvider);

        document.getElementById('btn-ingest-run').onclick = async () => {
            const input = document.getElementById('job-input').value.trim();
            if (!input) return;
            const btn = document.getElementById('btn-ingest-run');
            btn.disabled = true;

            const delivs = {};
            document.querySelectorAll('#deliv-selector-ingest .llm-pill').forEach(p => {
                delivs[p.dataset.deliv] = p.classList.contains('active');
            });

            const config = { 
                resume: delivs.resume, 
                cover: delivs.cover, 
                analysis: delivs.restitution 
            };
            const profil_id = activeProfilId;
            try {
                const res = await fetch('/api/ingest', { 
                    method: 'POST', 
                    headers: { 'Content-Type': 'application/json' }, 
                    body: JSON.stringify({ input, config, profil_id, llm_provider: selectedLlmProvider }) 
                });
                if (res.ok) { activeJobId = null; await loadOffers(); switchView('app'); }
            } catch (e) {}
            finally { 
                btn.disabled = false; 
                btn.innerHTML = `<span data-i18n="generate_app">${i18n.translations[i18n.current].generate_app}</span>`;
            }
        };

        document.querySelectorAll('.tab').forEach(tab => {
            tab.onclick = () => {
                document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                activeTab = tab.dataset.target;
                updateIframe();
            };
        });

        window.i18n = i18n;
        window.updateUIStrings = updateUIStrings;
        window.switchView = switchView;
        window.activeJobId = null;
        window.activeProfilId = null;
        window.activeTab = 'restitution';
        window.aiChatAttachments = [];
        window.syncLlmSelectors = syncLlmSelectors;
        window.selectedLlmProvider = selectedLlmProvider;

        window.onload = () => {
            updateUIStrings();
            if (typeof loadProfile === 'function') loadProfile();
            if (typeof loadOffers === 'function') loadOffers();
            handleRouting();
        };
