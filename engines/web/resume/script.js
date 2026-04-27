async function loadCV() {
    try {
        const urlParams = new URLSearchParams(window.location.search);
        const jobId = urlParams.get('id');
        const t = Date.now();
        let dataPath = (jobId && jobId !== 'null') ? `../data/instances/${jobId}/resume.json?t=${t}` : `../data/templates/resume.json?t=${t}`;
        
        let response = await fetch(dataPath);
        if (!response.ok) {
            console.warn(`Instance ${jobId} not found, falling back to template.`);
            response = await fetch(`../data/templates/resume.json?t=${t}`);
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
        document.getElementById('email').textContent = data.profile.email;
        document.getElementById('phone').textContent = data.profile.phone;
        
        document.getElementById('website').textContent = data.profile.website;
        safeSetHref('website-link', "https://" + data.profile.website);
        
        document.getElementById('linkedin').textContent = data.profile.linkedin;
        safeSetHref('linkedin-link', "https://www.linkedin.com/" + data.profile.linkedin);
        
        document.getElementById('github').textContent = data.profile.github;
        safeSetHref('github-link', "https://" + data.profile.github);
        
        // Labels and Sections
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
            
        // Skills (FIXED)
        const skillsContainer = document.getElementById('skills-container');
        skillsContainer.innerHTML = '';
        data.skills.forEach(cat => {
            const div = document.createElement('div');
            div.className = 'skill-category';
            div.innerHTML = `<h4>${cat.category}</h4><div class="skill-items">${cat.items.join(', ')}</div>`;
            skillsContainer.appendChild(div);
        });

        // Languages (FIXED)
        const langContainer = document.getElementById('languages-container');
        langContainer.innerHTML = '';
        data.languages.forEach(lang => {
            const div = document.createElement('div');
            div.className = 'contact-item'; 
            div.innerHTML = `<strong>${lang.name} :</strong> ${lang.level}`;
            langContainer.appendChild(div);
        });

        // Experiences & Projects (FIXED)
        const expContainer = document.getElementById('experiences-container');
        expContainer.innerHTML = '';
        data.experiences.forEach(exp => {
            const div = document.createElement('div');
            div.className = 'exp-item';
            
            if (exp.role) {
                // Style Emploi
                div.innerHTML = `
                    <h4>${exp.role}</h4>
                    <div class="company">${exp.company}</div>
                    <div class="period">${exp.period}</div>
                    <ul>${exp.description.map(line => `<li>${line}</li>`).join('')}</ul>
                `;
            } else {
                // Style Projet/Hackathon (Gras-Italique, Inline)
                div.innerHTML = `
                    <div class="project-line">
                        <span class="company-bold">${exp.company}</span> <span class="period">(${exp.period})</span>
                    </div>
                    <ul>${exp.description.map(line => `<li>${line}</li>`).join('')}</ul>
                `;
            }
            expContainer.appendChild(div);
        });

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

