import { clear, el, svg, text } from '../assets/js/dom.js';
import {
    safeSetHref,
    canonicalEmailHref,
    canonicalPhoneHref,
    canonicalWebsiteUrl,
    canonicalLinkedinUrl,
    canonicalGithubUrl,
    formatPhoneDisplay,
    firstCanonicalUrl,
} from '../assets/js/utils.js';

// Local flag removed in favor of localStorage
window.handleGenerate = () => {
    const urlParams = new URLSearchParams(window.location.search);
    // Use the offer slug as the canonical key — it matches activeJobId in the dashboard
    // and the generating_target_ localStorage key used by view.js and background_poll.
    const offerSlug = urlParams.get('offer');
    const jobId = offerSlug || urlParams.get('id') || urlParams.get('instance');
    if (!jobId) return;
    window.parent.triggerGeneration(jobId, null, { resume: true });
};

function showGenerating() {
    const gen = document.getElementById('generating-state');
    const con = document.getElementById('cv-container');
    const emp = document.getElementById('empty-state');
    if (gen) gen.style.display = 'flex';
    if (con) con.style.display = 'none';
    if (emp) emp.style.display = 'none';
    if (window.lucide) lucide.createIcons();
}

function showContent() {
    const gen = document.getElementById('generating-state');
    const con = document.getElementById('cv-container');
    const emp = document.getElementById('empty-state');
    if (gen) gen.style.display = 'none';
    if (con) con.style.display = 'block';
    if (emp) emp.style.display = 'none';
}

async function loadCV() {
    const urlParams = new URLSearchParams(window.location.search);
    const jobIdInUrl = urlParams.get('id') || urlParams.get('instance');
    const offerId = urlParams.get('offer');
    
    const jobId = window._currentJobId || jobIdInUrl;
    if (!jobId) return;

    try {
        const hasInstance = !!(jobId && jobId !== 'null');
        if (!hasInstance) return renderEmptyResumeState(null);

        let resInstance = await fetch(`/api/instances/${jobId}?t=${Date.now()}`);
        if (!resInstance.ok && !jobId.includes('__')) {
            resInstance = await fetch(`/api/offres/${jobId}/instance?t=${Date.now()}`);
        }

        if (!resInstance.ok) return renderEmptyResumeState(jobId);
        const instance = await resInstance.json();
        const status = instance.status.toLowerCase();

        let genTarget = { resume: false }; 
        const storageKey = offerId || jobIdInUrl;
        try {
            const stored = localStorage.getItem('generating_target_' + storageKey);
            if (stored) genTarget = JSON.parse(stored);
        } catch(e) {}

        if (instance.resume_json) {
            if (genTarget.resume) {
                genTarget.resume = false;
                localStorage.setItem('generating_target_' + storageKey, JSON.stringify(genTarget));
            }
            
            const errorBanner = document.getElementById('error-banner');
            if (errorBanner) errorBanner.style.display = (status === 'failed') ? 'block' : 'none';

            const fallbackContact = await fetchActiveProfileContact();
            showContent();
            renderTemplateResume(instance.resume_json, fallbackContact);
            applyPreviewScale();
        } else if (genTarget.resume) {
            if (status === 'failed') {
                genTarget.resume = false;
                localStorage.setItem('generating_target_' + storageKey, JSON.stringify(genTarget));
                renderEmptyResumeState(jobId, false);
            } else {
                showGenerating();
            }
        } else {
            renderEmptyResumeState(jobId, false);
        }
    } catch (e) {
        console.error("Unable to load CV:", e);
    }
}

async function fetchActiveProfileContact() {
    try {
        const res = await fetch(`/api/profile/active?t=${Date.now()}`);
        if (!res.ok) return null;
        const profil = await res.json();
        const p = profil?.content?.profile || {};
        return {
            localisation: p.location || '',
            telephone: p.phone || '',
            email: p.email || '',
            site_web: p.website || '',
            linkedin: p.linkedin || '',
            github: p.github || ''
        };
    } catch (_) {
        return null;
    }
}

// Reactive UI updates
window.addEventListener('storage', (e) => {
    const urlParams = new URLSearchParams(window.location.search);
    const storageKey = urlParams.get('offer') || urlParams.get('id') || urlParams.get('instance');
    if (e.key === 'generating_target_' + storageKey) {
        loadCV();
    }
});

function renderEmptyResumeState(jobId, isInstanceGenerating = false) {
    const stage = document.getElementById('empty-state');
    const gen = document.getElementById('generating-state');
    const con = document.getElementById('cv-container');
    if (gen) gen.style.display = 'none';
    if (con) con.style.display = 'none';
    if (!stage) return;
    stage.style.display = 'flex';

    const hasGenerateAction = !!jobId;
        
    clear(stage).appendChild(el('div', {
        style: 'display:flex; flex-direction:column; align-items:center;'
    }, [
        el('div', {
            style: 'width:64px; height:64px; background:#eff6ff; border-radius:50%; display:flex; align-items:center; justify-content:center; margin-bottom:24px; color:#0052ff;'
        }, [
            svg('svg', {
                xmlns: 'http://www.w3.org/2000/svg',
                fill: 'none',
                viewBox: '0 0 24 24',
                'stroke-width': '1.5',
                stroke: 'currentColor',
                style: 'width:32px; height:32px;'
            }, [svg('path', {
                'stroke-linecap': 'round',
                'stroke-linejoin': 'round',
                d: 'M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z'
            })])
        ]),
        el('h2', {
            style: 'font-size:20px; font-weight:700; color:#1e293b; margin-bottom:12px;',
            text: 'CV non disponible'
        }),
        hasGenerateAction ? el('button', {
            id: 'btn-generate-cv',
            style: 'background:#0052ff; color:white; border:none; padding:14px 32px; border-radius:12px; font-weight:600; cursor:pointer; font-size:15px; box-shadow:0 4px 12px rgba(0,82,255,0.2); transition:all 0.2s;',
            text: 'Générer le CV',
            onclick: () => window.handleGenerate()
        }) : null
    ]));
    if (window.lucide) lucide.createIcons();
}

function renderTemplateResume(data, fallbackContact = null) {
    if (!data || !data.identite) {
        console.error("Données de CV invalides ou incomplètes", data);
        return;
    }

    const cvContact = data.contact || {};
    const websiteUrl = firstCanonicalUrl(
        [cvContact.site_web, fallbackContact?.site_web],
        canonicalWebsiteUrl
    );
    const linkedinUrl = firstCanonicalUrl(
        [cvContact.linkedin, fallbackContact?.linkedin],
        canonicalLinkedinUrl
    );
    const githubUrl = firstCanonicalUrl(
        [cvContact.github, fallbackContact?.github],
        canonicalGithubUrl
    );

    const mergedContact = {
        localisation: fallbackContact?.localisation || cvContact.localisation || '',
        telephone: cvContact.telephone || fallbackContact?.telephone || '',
        email: cvContact.email || fallbackContact?.email || '',
        site_web: websiteUrl === '#' ? '' : websiteUrl,
        linkedin: linkedinUrl === '#' ? '' : linkedinUrl,
        github: githubUrl === '#' ? '' : githubUrl
    };

    // Profile Basic
    const upperName = String(data.identite.nom_complet || "").toLocaleUpperCase('fr-FR');
    const upperTitle = String(data.accroche.titre || "").toLocaleUpperCase('fr-FR');
    document.getElementById('name').textContent = upperName;
    document.getElementById('title').textContent = upperTitle;
    let pitch = data.accroche.paragraphe || "";
    // Audit & fix redundancy: strip "Titre: ..." if present
    pitch = pitch.replace(/^Titre\s*:\s*/i, "");
    // If the pitch still starts with the title, strip it
    if (upperTitle && pitch.toUpperCase().startsWith(upperTitle)) {
        pitch = pitch.substring(upperTitle.length).replace(/^[\s,—:.-]+/, "");
    }
    document.getElementById('pitch').textContent = pitch;
    
    const profileImg = document.getElementById('profile-img');
    if (profileImg) {
        // Normalize legacy URL: old instances may have /api/profile/photo instead of /api/profile/active/photo
        let photoUrl = data.identite.photo_url || 'assets/profile-picture.jpg';
        if (photoUrl === '/api/profile/photo') photoUrl = '/api/profile/active/photo';
        profileImg.src = photoUrl;
        profileImg.onerror = () => { profileImg.src = 'assets/profile-picture.jpg'; };
    }

    const setT = (id, val) => { const e = document.getElementById(id); if(e) e.textContent = val || ""; };

    // Sidebar Headers
    setT('contact-title', 'CONTACT');
    setT('skills-title', 'COMPÉTENCES');
    setT('languages-title', 'LANGUES');

    // Sidebar Contact
    const formattedPhone = formatPhoneDisplay(mergedContact.telephone);
    setT('location', mergedContact.localisation);
    setT('email', mergedContact.email);
    setT('phone', formattedPhone);
    setT('website', 'Site web');
    setT('linkedin', 'LinkedIn');
    setT('github', 'GitHub');

    // Main Headers
    setT('experiences-title', 'EXPÉRIENCES');
    setT('education-title', 'FORMATIONS');

    // Links
    safeSetHref('email-link', canonicalEmailHref(mergedContact.email));
    safeSetHref('phone-link', canonicalPhoneHref(mergedContact.telephone));
    safeSetHref('website-link', websiteUrl);
    safeSetHref('linkedin-link', linkedinUrl);
    safeSetHref('github-link', githubUrl);

    // Header Meta
    const durationEl = document.getElementById('duration');
    const rhythmEl = document.getElementById('rhythm');
    const durationLabel = document.getElementById('duration-label');
    const rhythmLabel = document.getElementById('rhythm-label');

    if (data.accroche.duree) {
        if (durationLabel) durationLabel.textContent = 'Durée :';
        if (durationEl) durationEl.textContent = data.accroche.duree;
        durationEl?.parentElement?.classList.remove('hidden');
    } else {
        durationEl?.parentElement?.classList.add('hidden');
    }

    if (data.accroche.rythme) {
        if (rhythmLabel) rhythmLabel.textContent = 'Rythme :';
        if (rhythmEl) rhythmEl.textContent = data.accroche.rythme;
        rhythmEl?.parentElement?.classList.remove('hidden');
    } else {
        rhythmEl?.parentElement?.classList.add('hidden');
    }

    // Skills
    const skillsContainer = document.getElementById('skills-container');
    if (skillsContainer) {
        clear(skillsContainer);
        (data.competences || []).forEach((cat) => {
            skillsContainer.appendChild(el('div', { className: 'skill-category' }, [
                el('h4', { text: cat.categorie }),
                el('div', { className: 'skill-items', text: (cat.items || []).join(', ') }),
            ]));
        });
    }

    // Languages
    const langContainer = document.getElementById('languages-container');
    if (langContainer) {
        clear(langContainer);
        (data.langues || []).forEach((lang) => {
            const level = formatLanguageLevel(lang.niveau);
            langContainer.appendChild(el('div', { className: 'contact-item' }, [
                el('strong', { text: `${lang.langue} :` }),
                text(` ${level}`),
            ]));
        });
    }

    // Experiences
    const expContainer = document.getElementById('experiences-container');
    if (expContainer) {
        clear(expContainer);
        (data.experiences || []).forEach(exp => {
            expContainer.appendChild(createExperienceBlock({
                title: exp.poste,
                sub: exp.entreprise,
                period: exp.periode,
                description: exp.bullets || [],
            }));
        });
    }

    // Projects
    const projContainer = document.getElementById('projects-container');
    if (projContainer) {
        clear(projContainer);
        (data.projets || []).forEach(proj => {
            projContainer.appendChild(createExperienceBlock({
                title: proj.nom,
                sub: "",
                period: proj.periode,
                description: proj.bullets || [],
                project: true,
            }));
        });
    }

    // Education
    const eduContainer = document.getElementById('education-container');
    if (eduContainer) {
        clear(eduContainer);
        (data.formations || []).forEach(edu => {
            eduContainer.appendChild(el('div', { className: 'edu-item' }, [
                el('strong', { text: edu.etablissement }),
                edu.periode ? el('span', { className: 'period', text: ` (${edu.periode})` }) : null,
                el('span', { className: 'degree', text: edu.diplome }),
            ]));
        });
    }

    if (window.lucide) lucide.createIcons();
}

function createExperienceBlock({ title, sub, period, description, project = false }) {
    const titleRow = project
        ? el('h4', {}, [
            text(title),
            el('span', {
                style: 'font-style:italic; font-weight:normal; font-size:0.75rem; color:var(--cv-muted); margin-left:6px; display:inline-block;',
                text: ` (${period})`,
            }),
        ])
        : el('h4', { text: title });

    return el('div', { className: 'exp-item' }, [
        titleRow,
        sub ? el('div', { className: 'company', text: sub }) : null,
        !project ? el('div', { className: 'period', style: 'margin-top:1px; margin-bottom:3px;', text: period }) : null,
        el('ul', {}, description.map((line) => el('li', { text: line }))),
    ]);
}

function createEducationBlock(edu) {
    const degrees = (edu.degree || '').split('\n').map(d => d.trim()).filter(Boolean);
    
    return el('div', { className: 'edu-item' }, [
        el('strong', { text: edu.school }),
        edu.period ? text(` (${edu.period})`) : null,
        ...degrees.map(d => el('span', { className: 'degree', text: d })),
    ]);
}

function formatLanguageLevel(value) {
    const raw = String(value || '').trim();
    if (!raw) return '';
    if (raw.toLowerCase() === 'natif') return 'Natif';
    return raw;
}

function applyPreviewScale() {
    if (window.self === window.top) return;

    const page = document.querySelector('.a4-page');
    const stage = document.querySelector('.page-stage');
    if (!page || !stage) return;

    page.style.transform = 'none';
    page.style.left = '0px';
    page.style.top = '0px';

    requestAnimationFrame(() => {
        const availableWidth = window.innerWidth - 40;
        const availableHeight = window.innerHeight - 40;
        const pageWidth = page.offsetWidth;
        const pageHeight = page.offsetHeight;
        
        // Calcul identique pour un rendu uniforme
        const scale = Math.min(1, availableWidth / pageWidth, availableHeight / pageHeight);
        const scaledWidth = pageWidth * scale;
        const scaledHeight = pageHeight * scale;
        
        const offsetX = Math.max(0, (availableWidth - scaledWidth) / 2) + 20;
        const offsetY = 20;

        page.style.transformOrigin = "top center";
        page.style.transform = `translateX(-50%) scale(${scale})`;
        page.style.left = "50%";
        page.style.top = `${offsetY}px`;
        if (stage) stage.style.height = `${window.innerHeight}px`;
    });
}

// document.getElementById('download-pdf').addEventListener('click', () => window.print());
window.addEventListener('DOMContentLoaded', () => {
    if (window.self !== window.top) {
        document.body.classList.add('is-framed');
    }
    loadCV();
    applyPreviewScale();
});
window.addEventListener('resize', applyPreviewScale);
