async function loadCV() {
    try {
        const urlParams = new URLSearchParams(window.location.search);
        const jobId = urlParams.get('id');
        const t = Date.now();
        let dataPath = (jobId && jobId !== 'null') ? `/api/instances/${jobId}/resume` : `/api/profile/active/resume?t=${t}`;
        
        let response = await fetch(dataPath);
        if (!response.ok) {
            console.warn(`Data not found at ${dataPath}, falling back to template.`);
            response = await fetch(`/templates/resume.json?t=${t}`);
        }
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }

        const data = await response.json();
        
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
            div.innerHTML = `
                <h4>${exp.role}</h4>
                <div class="company">${exp.company}</div>
                <div class="period">${exp.period}</div>
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
                div.className = 'exp-item'; // Re-use styling
                div.innerHTML = `
                    <h4>${proj.role}</h4>
                    <div class="company">${proj.company}</div>
                    <div class="period">${proj.period}</div>
                    <ul>${proj.description.map(line => `<li>${line}</li>`).join('')}</ul>
                `;
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
    } catch (error) { console.error(error); }
}

function safeSetHref(id, url) {
    const el = document.getElementById(id);
    if (el) el.href = url;
}

document.getElementById('download-pdf').addEventListener('click', () => window.print());
window.addEventListener('DOMContentLoaded', () => {
    if (window.self !== window.top) {
        document.body.classList.add('is-framed');
    }
    loadCV();
});
