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

export async function generateApplication(jobId, provider, options = {}) {
    const query = new URLSearchParams({
        llm_provider: provider,
        restitution: options.restitution ?? true,
        resume: options.resume ?? true,
        cover_letter: options.cover_letter ?? true
    });
    const res = await fetch(`/api/instances/${jobId}/generate?${query}`, { method: 'POST' });
    if (!res.ok) throw new Error('Generation failed');
    return { slug: jobId }; // The backend returns 202 Accepted
}

export async function updateAnnexe(_id, _payload) {
    // Endpoint non expose pour l'instant; on garde la fonction pour l'API UI.
    return Promise.resolve();
}
