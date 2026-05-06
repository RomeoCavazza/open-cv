import * as api from '../api.js';
import * as ui from '../ui.js';
import { clear, safeClick } from '../dom.js';
import { EVENTS, emit } from '../modules/events.js';
import { 
    setActiveProfilId, 
    setLoadedProfileImage, 
    setLoadedProfileExtras,
    loadedProfileImage,
    loadedProfileExtras
} from '../state.js';

export class ProfileController {
    constructor() {
        console.log("[ProfileController] Initialized");
    }

    async loadProfile() {
        try {
            const profileResponse = await api.fetchProfile();
            const content = profileResponse.content;
            setActiveProfilId(profileResponse.id || null);

            setLoadedProfileExtras(Object.fromEntries(
                Object.entries(content).filter(([key]) => ![
                    'profile', 'apprenticeship', 'experiences', 'projects',
                    'education', 'languages', 'skills', 'labels', 'documents'
                ].includes(key))
            ));

            const setVal = (id, val) => { const el = document.getElementById(id); if (el) el.value = val || ""; };
            setVal('prof-firstname', content.profile?.firstname);
            setVal('prof-lastname', content.profile?.lastname);
            setVal('prof-title', content.profile?.title);
            setVal('prof-offer-type', content.profile?.offer_type || "Alternance");
            setVal('prof-duration', content.apprenticeship?.duration);
            setVal('prof-rhythm', content.apprenticeship?.rhythm);
            setVal('prof-pitch', content.profile?.pitch);
            setVal('prof-location', content.profile?.location);
            setVal('prof-phone', content.profile?.phone);
            setVal('prof-email', content.profile?.email);
            setVal('prof-linkedin', content.profile?.linkedin);
            setVal('prof-website', content.profile?.website);
            setVal('prof-github', content.profile?.github);

            setVal('prof-resume-template', ui.stringifyDocument(content.documents?.resume_template || {}));
            setVal('prof-cover-letter-template', ui.stringifyDocument(content.documents?.cover_letter_template || {}));

            setLoadedProfileImage(content.profile?.image || "");
            setVal('prof-image-base64', ""); // Always clear hidden input after load, original is in state

            const preview = document.getElementById('prof-image-preview');
            const placeholder = document.getElementById('preview-placeholder');
            if (preview && content.profile?.image) {
                const imageUrl = (content.profile.image === "persisted:bytea")
                    ? `/api/profile/active/photo?t=${Date.now()}`
                    : content.profile.image;
                preview.style.backgroundImage = `url("${imageUrl}")`;
                if (placeholder) placeholder.style.display = 'none';
            } else if (preview) {
                preview.style.backgroundImage = '';
                if (placeholder) placeholder.style.display = 'block';
            }

            ui.renderList('list-experiences', content.experiences || [], ui.createExpRow);
            ui.renderList('list-projects', content.projects || [], ui.createExpRow);
            ui.renderList('list-education', content.education || [], ui.createEduRow);
            ui.renderList('list-languages', content.languages || [], ui.createLangRow);
            ui.renderList('list-skills', content.skills || [], ui.createSkillRow);

            try {
                const annexes = await api.fetchAnnexes();
                ui.renderList('list-annexes', annexes || [], ui.createAnnexeRow);
            } catch (annexError) {
                console.error("Échec du chargement des annexes", annexError);
            }

            ui.updateUIStrings();
        } catch (e) { console.warn("Profile load failed", e); }
    }

    async saveProfile() {
        const btn = document.getElementById('btn-save-profile');
        if (!btn) return;
        
        const oldText = btn.textContent;
        btn.disabled = true;
        btn.textContent = '...';
        try {
            const data = {
                ...loadedProfileExtras,
                profile: {
                    firstname: document.getElementById('prof-firstname').value,
                    lastname: document.getElementById('prof-lastname').value,
                    title: document.getElementById('prof-title').value,
                    offer_type: document.getElementById('prof-offer-type').value,
                    pitch: document.getElementById('prof-pitch').value,
                    location: document.getElementById('prof-location').value,
                    phone: document.getElementById('prof-phone').value,
                    email: document.getElementById('prof-email').value,
                    linkedin: document.getElementById('prof-linkedin').value,
                    website: document.getElementById('prof-website').value,
                    github: document.getElementById('prof-github').value,
                    image: document.getElementById('prof-image-base64').value || loadedProfileImage || "",
                },
                apprenticeship: {
                    duration: document.getElementById('prof-duration').value,
                    rhythm: document.getElementById('prof-rhythm').value,
                },
                experiences: Array.from(document.querySelectorAll('#list-experiences .form-row-exp')).map(r => ({
                    role: r.querySelector('.exp-role').value,
                    company: r.querySelector('.exp-company').value,
                    period: r.querySelector('.exp-period').value,
                    description: r.querySelector('.exp-desc').value.split('\n').filter(Boolean),
                })),
                projects: Array.from(document.querySelectorAll('#list-projects .form-row-exp')).map(r => ({
                    role: r.querySelector('.exp-role').value,
                    company: r.querySelector('.exp-company').value,
                    period: r.querySelector('.exp-period').value,
                    description: r.querySelector('.exp-desc').value.split('\n').filter(Boolean),
                })),
                education: Array.from(document.querySelectorAll('.form-row-edu')).map(r => ({
                    school: r.querySelector('.edu-school').value,
                    degree: r.querySelector('.edu-degree').value,
                    period: r.querySelector('.edu-period').value,
                })),
                languages: Array.from(document.querySelectorAll('.form-row-lang')).map(r => ({
                    name: r.querySelector('.lang-name').value,
                    level: r.querySelector('.lang-level').value,
                })),
                skills: Array.from(document.querySelectorAll('.skill-cat-row')).map(r => ({
                    category: r.querySelector('.skill-cat-name').value,
                    items: Array.from(r.querySelectorAll('.skill-text')).map(s => s.textContent),
                })),
                documents: {
                    resume_template: JSON.parse(document.getElementById('prof-resume-template').value || "{}"),
                    cover_letter_template: JSON.parse(document.getElementById('prof-cover-letter-template').value || "{}"),
                }
            };
            await api.saveProfile(data);

            const annexeRows = Array.from(document.querySelectorAll('.form-row-annexe'));
            for (const row of annexeRows) {
                if (row.dataset.markedForDeletion === "true") {
                    await api.deleteAnnexe(row.dataset.fileId);
                } else if (!row.dataset.fileId && row.dataset.fileData) {
                    await api.uploadAnnexe({
                        label: row.querySelector('.annexe-name').value,
                        filename: row.dataset.fileName,
                        content_type: row.dataset.fileType,
                        data_url: row.dataset.fileData
                    });
                } else if (row.dataset.fileId) {
                    await api.updateAnnexe(row.dataset.fileId, {
                        label: row.querySelector('.annexe-name').value
                    });
                }
            }

            await this.loadProfile();
            emit(EVENTS.PROFILE_UPDATED);
            alert('Profil sauvegardé !');
        } catch (e) { 
            const errorMsg = e.message.includes("Unexpected token") ? "Format JSON invalide dans les templates" : "Erreur sauvegarde";
            alert(errorMsg); 
            console.error(e); 
        }
        finally { btn.disabled = false; btn.textContent = oldText; }
    }

    attachEventListeners() {
        safeClick('btn-save-profile', () => this.saveProfile());
        safeClick('prof-image-preview', () => {
            const input = document.getElementById('prof-image-file');
            if (input) input.click();
        });

        const profFile = document.getElementById('prof-image-file');
        if (profFile) profFile.onchange = async (e) => {
            const file = e.target.files[0];
            if (!file) return;
            const b64 = await ui.readFileAsDataUrl(file);
            setLoadedProfileImage(b64);
            const inputBase64 = document.getElementById('prof-image-base64');
            if (inputBase64) inputBase64.value = b64;
            const preview = document.getElementById('prof-image-preview');
            if (preview) {
                preview.style.backgroundImage = `url(${b64})`;
                const placeholder = document.getElementById('preview-placeholder');
                if (placeholder) placeholder.style.display = 'none';
            }
        };
    }
}
