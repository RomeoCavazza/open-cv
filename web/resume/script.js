async function loadCV() {
    try {
        const urlParams = new URLSearchParams(window.location.search);
        const jobId = urlParams.get('id') || urlParams.get('instance');
        const hasInstance = !!(jobId && jobId !== 'null');

        if (!hasInstance) {
            renderEmptyResumeState(null);
            return;
        }

        const response = await fetch(`/api/instances/${jobId}/resume`);

        if (!response.ok) {
            renderEmptyResumeState(jobId);
            return;
        }

        const data = await response.json();
        renderTemplateResume(data);
    } catch (error) { console.error(error); }
}

function renderEmptyResumeState(jobId) {
    const stage = document.querySelector('.page-stage');
    if (!stage) return;

    const hasGenerateAction = !!jobId;
    stage.innerHTML = `
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: flex-start; height: 100vh; width: 100%; padding-top: 18vh; padding-left: 40px; padding-right: 40px; text-align: center; color: #64748b; background: #fff;">
            <div style="width: 64px; height: 64px; background: #eff6ff; border-radius: 50%; display: flex; align-items: center; justify-content: center; margin-bottom: 24px; color: #0052ff;">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" style="width: 32px; height: 32px;">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                </svg>
            </div>
            <h2 style="font-size: 20px; font-weight: 700; color: #1e293b; margin-bottom: 12px;">CV non disponible</h2>
            ${hasGenerateAction ? `<button id="btn-generate-cv" style="
                background: #0052ff;
                color: white;
                border: none;
                padding: 14px 32px;
                border-radius: 12px;
                font-weight: 600;
                cursor: pointer;
                font-size: 15px;
                box-shadow: 0 4px 12px rgba(0,82,255,0.2);
                transition: all 0.2s;
            ">Générer le CV</button>` : ''}
        </div>
    `;
    if (window.lucide) lucide.createIcons();
    // No applyPreviewScale here as we want full-width/height

    if (hasGenerateAction) {
        document.getElementById('btn-generate-cv').onclick = async () => {
            const btn = document.getElementById('btn-generate-cv');
            btn.disabled = true;
            btn.innerText = 'Génération...';
            try {
                const res = await fetch(`/api/instances/${jobId}/generate`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ deliverables: { resume: true, restitution: false, cover: false } })
                });
                if (res.ok) window.location.reload();
                else btn.disabled = false;
            } catch (e) {
                btn.disabled = false;
            }
        };
    }
}

function renderTemplateResume(data) {

    // Profile Basic
    document.getElementById('name').textContent = data.profile.name;
    document.getElementById('title').textContent = data.profile.title;
    document.getElementById('pitch').textContent = data.profile.pitch;
    document.getElementById('profile-img').src = data.profile.image;

    // Sidebar Contact
    document.getElementById('location').textContent = data.profile.location;
    document.getElementById('contact-title').textContent = data.labels.contact;
    document.getElementById('skills-title').textContent = data.labels.skills;
    document.getElementById('languages-title').textContent = data.labels.languages;
    document.getElementById('experiences-title').textContent = data.labels.experiences;
    if (document.getElementById('projects-title')) document.getElementById('projects-title').textContent = data.labels.projects || "PROJETS";
    document.getElementById('education-title').textContent = data.labels.education;
    document.getElementById('email').textContent = data.profile.email;
    document.getElementById('phone').textContent = data.profile.phone;

    // Links (Data only for href)
    safeSetHref('website-link', "https://" + data.profile.website);
    safeSetHref('linkedin-link', "https://www.linkedin.com/" + data.profile.linkedin);
    safeSetHref('github-link', "https://" + data.profile.github);

    // Labels Overwrite
    if (data.labels) {
        const labelMap = {
            'duration-label': 'duration',
            'rhythm-label': 'rhythm',
            'contact-title': 'contact',
            'skills-title': 'skills',
            'languages-title': 'languages',
            'experiences-title': 'experiences',
            'education-title': 'education',
            'website': 'website',
            'linkedin': 'linkedin',
            'github': 'github'
        };
        Object.entries(labelMap).forEach(([id, key]) => {
            const el = document.getElementById(id);
            if (el) el.textContent = data.labels[key];
        });

        const downloadBtn = document.getElementById('download-pdf');
        if (downloadBtn && data.labels.download) {
            const icon = downloadBtn.querySelector('i');
            downloadBtn.textContent = '';
            if (icon) downloadBtn.appendChild(icon);
            downloadBtn.appendChild(document.createTextNode(' ' + data.labels.download));
        }
    }

    // Header Meta
    document.getElementById('duration').textContent = data.apprenticeship.duration + ' — à partir de ' + data.apprenticeship.start;
    document.getElementById('rhythm').textContent = data.apprenticeship.rhythm;

    // Skills
    const skillsContainer = document.getElementById('skills-container');
    skillsContainer.innerHTML = '';
    data.skills.forEach(cat => {
        const div = document.createElement('div');
        div.className = 'skill-category';
        div.innerHTML = `<h4>${cat.category}</h4><div class="skill-items">${cat.items.join(', ')}</div>`;
        skillsContainer.appendChild(div);
    });

    // Languages
    const langContainer = document.getElementById('languages-container');
    langContainer.innerHTML = '';
    data.languages.forEach(lang => {
        const div = document.createElement('div');
        div.className = 'contact-item';
        div.innerHTML = `<strong>${lang.name} :</strong> ${lang.level}`;
        langContainer.appendChild(div);
    });

    // Experiences
    const expContainer = document.getElementById('experiences-container');
    expContainer.innerHTML = '';
    (data.experiences || []).forEach(exp => {
        const div = document.createElement('div');
        div.className = 'exp-item';
        const title = exp.role || exp.company;
        const sub = exp.role ? exp.company : "";
        div.innerHTML = `
                <h4>${title}</h4>
                ${sub ? `<div class="company">${sub}</div>` : ''}
                <div class="period" style="margin-top: 1px; margin-bottom: 3px;">${exp.period}</div>
                <ul>${exp.description.map(line => `<li>${line}</li>`).join('')}</ul>
            `;
        expContainer.appendChild(div);
    });

    // Projects
    const projContainer = document.getElementById('projects-container');
    if (projContainer) {
        projContainer.innerHTML = '';
        (data.projects || []).forEach(proj => {
            const div = document.createElement('div');
            div.className = 'exp-item';
            const title = proj.role || proj.company;
            const sub = proj.role ? proj.company : "";
            div.innerHTML = `<h4>${title}<span style="font-style: italic; font-weight: normal; font-size: 0.75rem; color: var(--cv-muted); margin-left: 6px; display: inline-block;"> (${proj.period})</span></h4>` +
                (sub ? `<div class="company">${sub}</div>` : '') +
                `<ul>${proj.description.map(line => `<li>${line}</li>`).join('')}</ul>`;
            projContainer.appendChild(div);
        });
    }

    // Education
    const eduContainer = document.getElementById('education-container');
    eduContainer.innerHTML = '';
    data.education.forEach(edu => {
        const div = document.createElement('div');
        div.className = 'edu-item';
        div.innerHTML = `
                <strong>${edu.school}</strong> ${edu.period ? `<span class="period">(${edu.period})</span>` : ''}
                <span class="degree">${edu.degree}</span>
            `;
        eduContainer.appendChild(div);
    });

    lucide.createIcons();
}

function safeSetHref(id, url) {
    const el = document.getElementById(id);
    if (el) el.href = url;
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
        const availableWidth = window.innerWidth - 32;
        const availableHeight = window.innerHeight - 32;
        const pageWidth = page.offsetWidth;
        const pageHeight = page.offsetHeight;
        const scale = Math.min(1, availableWidth / pageWidth, availableHeight / pageHeight);
        const scaledWidth = pageWidth * scale;
        const scaledHeight = pageHeight * scale;
        const offsetX = Math.max(0, (availableWidth - scaledWidth) / 2);
        const offsetY = 16;

        page.style.transform = `scale(${scale})`;
        page.style.left = `${offsetX}px`;
        page.style.top = `${offsetY}px`;
        stage.style.height = `${window.innerHeight}px`;
    });
}

document.getElementById('download-pdf').addEventListener('click', () => window.print());
window.addEventListener('DOMContentLoaded', () => {
    if (window.self !== window.top) {
        document.body.classList.add('is-framed');
    }
    loadCV();
    applyPreviewScale();
});
window.addEventListener('resize', applyPreviewScale);
