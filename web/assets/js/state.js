// Global State for RecruitAI Dashboard
export let i18n = {
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

export let activeJobId = null;
export let activeTab = 'restitution';
export let aiChatAttachments = [];
export let activeProfilId = null;
export let loadedProfileExtras = {};
export let loadedProfileImage = "";
export let loadedApprenticeshipCalendarDocument = null;
export let selectedLlmProvider = localStorage.getItem('recruitai_llm') || 'ollama';

export let collapsedOfferCategories = (() => {
    try {
        return JSON.parse(localStorage.getItem('recruitai_collapsed_offer_categories') || '[]');
    } catch (_) {
        return [];
    }
})();

export let offerFlags = (() => {
    try {
        return JSON.parse(localStorage.getItem('recruitai_offer_flags') || '{}');
    } catch (_) {
        return {};
    }
})();

// Setters
export function setActiveJobId(id) { activeJobId = id; }
export function setActiveTab(tab) { activeTab = tab; }
export function setActiveProfilId(id) { activeProfilId = id; }
export function setLoadedProfileImage(img) { loadedProfileImage = img; }
export function setLoadedApprenticeshipCalendarDocument(doc) { loadedApprenticeshipCalendarDocument = doc; }
export function setLoadedProfileExtras(extras) { loadedProfileExtras = extras; }
export function setSelectedLlmProvider(provider) { selectedLlmProvider = provider; }

export function saveOfferFlags() {
    localStorage.setItem('recruitai_offer_flags', JSON.stringify(offerFlags));
}

export function saveCollapsedCategories() {
    localStorage.setItem('recruitai_collapsed_offer_categories', JSON.stringify(collapsedOfferCategories));
}
