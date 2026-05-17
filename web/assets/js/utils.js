/**
 * Shared utilities for URL canonicalization and formatting.
 */

export function safeSetHref(id, url) {
    const el = document.getElementById(id);
    if (!el) return;
    const finalUrl = url || '#';
    if (finalUrl === '#') {
        el.removeAttribute('href');
        el.removeAttribute('target');
        el.removeAttribute('rel');
    } else {
        el.href = finalUrl;
        if (/^https?:\/\//i.test(finalUrl)) {
            el.setAttribute('target', '_blank');
            el.setAttribute('rel', 'noopener noreferrer');
        } else {
            el.removeAttribute('target');
            el.removeAttribute('rel');
        }
    }
}

export function parseHttpUrl(value) {
    if (!value) return null;
    const trimmed = String(value).trim();
    if (!trimmed) return null;

    try {
        return new URL(trimmed);
    } catch (_) {
        try {
            return new URL(`https://${trimmed}`);
        } catch (_) {
            return null;
        }
    }
}

export function firstCanonicalUrl(candidates, canonicalize) {
    const list = Array.isArray(candidates) ? candidates : [];
    for (const candidate of list) {
        const url = canonicalize(candidate);
        if (url && url !== '#') return url;
    }
    return '#';
}

export function canonicalWebsiteUrl(value) {
    const url = parseHttpUrl(value);
    if (!url) return '#';
    if (url.protocol !== 'http:' && url.protocol !== 'https:') return '#';
    return url.toString();
}

export function canonicalEmailHref(value) {
    const raw = String(value || '').trim();
    if (!raw) return '#';
    if (/^mailto:/i.test(raw)) return raw;
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(raw)) return '#';
    return `mailto:${raw}`;
}

export function normalizePhoneDigits(value) {
    const raw = String(value || '').trim();
    if (!raw) return '';
    const digits = raw.replace(/\D/g, '');
    if (!digits) return '';

    if (digits.startsWith('33') && digits.length === 11) {
        return `0${digits.slice(2)}`;
    }
    if (digits.length === 9) {
        return `0${digits}`;
    }
    if (digits.length === 10 && digits.startsWith('0')) {
        return digits;
    }
    return digits;
}

export function formatPhoneDisplay(value) {
    const normalized = normalizePhoneDigits(value);
    if (normalized.length === 10 && normalized.startsWith('0')) {
        return normalized.match(/.{1,2}/g).join('.');
    }
    return String(value || '').trim();
}

export function canonicalPhoneHref(value) {
    const raw = String(value || '').trim();
    if (!raw) return '#';

    const digits = raw.replace(/\D/g, '');
    if (!digits) return '#';

    if (digits.startsWith('33') && digits.length === 11) {
        return `tel:+33${digits.slice(2)}`;
    }
    if (digits.length === 10 && digits.startsWith('0')) {
        return `tel:+33${digits.slice(1)}`;
    }
    if (/^\+/.test(raw)) {
        return `tel:+${digits}`;
    }
    if (digits.length >= 6) {
        return `tel:${digits}`;
    }
    return '#';
}

export function canonicalLinkedinUrl(value) {
    const raw = String(value || '').trim();
    if (!raw) return '#';

    const nestedMatches = Array.from(raw.matchAll(/linkedin\.com\/in\/([^/?#]+)/ig));
    if (nestedMatches.length > 0) {
        const handle = nestedMatches[nestedMatches.length - 1][1];
        if (handle) return `https://www.linkedin.com/in/${handle}`;
    }

    const parsed = parseHttpUrl(raw);
    if (parsed && parsed.hostname.toLowerCase().includes('linkedin.')) {
        const path = parsed.pathname.replace(/^\/+/, '').replace(/\/+$/, '');
        const profiles = Array.from(path.matchAll(/(?:^|\/)in\/([^/?#]+)/ig));
        if (profiles.length > 0) {
            const handle = profiles[profiles.length - 1][1];
            if (handle) return `https://www.linkedin.com/in/${handle}`;
        }
        return path ? `https://www.linkedin.com/${path}` : 'https://www.linkedin.com/';
    }

    const profileFromRaw = Array.from(raw.matchAll(/(?:^|\/)in\/([^/?#]+)/ig));
    if (profileFromRaw.length > 0) {
        const handle = profileFromRaw[profileFromRaw.length - 1][1];
        if (handle) return `https://www.linkedin.com/in/${handle}`;
    }

    const handle = raw
        .replace(/^https?:\/\//i, '')
        .replace(/^www\./i, '')
        .replace(/^linkedin\.com\//i, '')
        .replace(/^in\//i, '')
        .replace(/^linkedin\.com\/in\//i, '')
        .replace(/^\/+/, '')
        .replace(/\/+$/, '');
    if (!handle) return '#';
    return `https://www.linkedin.com/in/${handle}`;
}

export function canonicalGithubUrl(value) {
    const raw = String(value || '').trim();
    if (!raw) return '#';

    const nestedMatches = Array.from(raw.matchAll(/github\.com\/([^/?#]+)/ig));
    if (nestedMatches.length > 0) {
        const handle = nestedMatches[nestedMatches.length - 1][1];
        if (handle && handle.toLowerCase() !== 'github.com') {
            return `https://github.com/${handle}`;
        }
    }

    const parsed = parseHttpUrl(raw);
    if (parsed && parsed.hostname.toLowerCase().includes('github.com')) {
        const path = parsed.pathname.replace(/^\/+/, '').replace(/\/+$/, '');
        const cleaned = path.replace(/^github\.com\//i, '');
        return cleaned ? `https://github.com/${cleaned}` : 'https://github.com/';
    }

    const accountFromRaw = raw.match(/github\.com\/([^/?#]+)/i);
    if (accountFromRaw && accountFromRaw[1]) {
        return `https://github.com/${accountFromRaw[1]}`;
    }

    const handle = raw
        .replace(/^https?:\/\//i, '')
        .replace(/^www\./i, '')
        .replace(/^github\.com\//i, '')
        .replace(/^github\.com\/github\.com\//i, '')
        .replace(/^\/+/, '')
        .replace(/\/+$/, '');
    if (!handle) return '#';
    return `https://github.com/${handle}`;
}
