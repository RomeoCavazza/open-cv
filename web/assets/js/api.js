// API Service for RecruitAI
export async function fetchProfile() {
    const res = await fetch(`/api/profile/active?t=${Date.now()}`, { cache: 'no-store' });
    if (!res.ok) throw new Error('Failed to fetch profile');
    return await res.json();
}

export async function saveProfile(content) {
    const res = await fetch('/api/profile/active', {
        method: 'PUT',
        cache: 'no-store',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(content)
    });
    if (!res.ok) {
        const errorText = await res.text();
        console.error("ERREUR SAUVEGARDE PROFIL:", errorText);
        throw new Error(errorText || `HTTP ${res.status}`);
    }
    return res;
}

export async function fetchOffers() {
    const res = await fetch('/api/offres');
    if (!res.ok) throw new Error('Failed to fetch offers');
    return await res.json();
}

export async function fetchAnnexes() {
    const res = await fetch('/api/profile/active/annexes');
    if (!res.ok) throw new Error('Failed to fetch annexes');
    return await res.json();
}

export async function uploadAnnexe(annexe) {
    const res = await fetch('/api/profile/active/annexes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(annexe)
    });
    if (!res.ok) throw new Error('Upload failed');
    return await res.json();
}

export async function deleteAnnexe(id) {
    const res = await fetch(`/api/profile/active/annexes/${id}`, { method: 'DELETE' });
    if (!res.ok) throw new Error('Delete failed');
    return res;
}

export async function ingestOffer(urlOrText) {
    const res = await fetch('/api/ingest', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ input: urlOrText })
    });
    if (!res.ok) throw new Error('Ingest failed');
    return await res.json();
}

export async function generateApplication(jobId, provider, options = {}, storageKey = null) {
    const key = storageKey || jobId;
    let targetId = jobId;

    // If it's an offer slug, resolve the instance slug first
    if (jobId && !jobId.includes('__')) {
        try {
            const res = await fetch(`/api/offres/${jobId}/instance`);
            if (res.ok) {
                const inst = await res.json();
                if (inst && inst.slug) targetId = inst.slug;
            }
        } catch(e) {}
    }

    const backendOpts = {
        restitution: options.restitution ?? false,
        resume: options.resume ?? false,
        cover_letter: options.cover_letter ?? false
    };

    let existingOpts = {};
    try {
        existingOpts = JSON.parse(localStorage.getItem('generating_target_' + key) || '{}');
    } catch(e) {}

    const uiOpts = {
        restitution: backendOpts.restitution || existingOpts.restitution || false,
        resume: backendOpts.resume || existingOpts.resume || false,
        cover_letter: backendOpts.cover_letter || existingOpts.cover_letter || false
    };
    
    try {
        localStorage.setItem('generating_target_' + key, JSON.stringify(uiOpts));
    } catch(e) {}
    
    const query = new URLSearchParams({
        llm_provider: provider || 'ollama',
        restitution: backendOpts.restitution,
        resume: backendOpts.resume,
        cover_letter: backendOpts.cover_letter
    });

    console.log(`[API] Starting generation for ${targetId} (key: ${key})`, backendOpts);
    const res = await fetch(`/api/instances/${targetId}/generate?${query}`, { method: 'POST' });
    if (!res.ok) {
        const err = await res.text();
        console.error(`[API] Generation failed for ${targetId}:`, err);
        throw new Error('Generation failed: ' + err);
    }
    return { slug: targetId };
}

export async function updateAnnexe(_id, _payload) {
    // Endpoint non expose pour l'instant; on garde la fonction pour l'API UI.
    return Promise.resolve();
}
