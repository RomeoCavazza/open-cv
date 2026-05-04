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

export async function runIngest(payload) {
    const res = await fetch('/api/ingest', { 
        method: 'POST', 
        headers: { 'Content-Type': 'application/json' }, 
        body: JSON.stringify(payload) 
    });
    if (!res.ok) throw new Error('Ingest failed');
    return res;
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

export function getAnnexeUrl(id) {
    return `/api/profile/active/annexes/${id}`;
}
