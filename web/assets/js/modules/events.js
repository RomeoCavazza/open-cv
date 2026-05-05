/**
 * Simple event bus for RecruitAI
 */
export const EVENTS = {
    OFFER_SELECTED: 'OFFER_SELECTED',
    INGEST_COMPLETED: 'INGEST_COMPLETED',
    PROFILE_UPDATED: 'PROFILE_UPDATED'
};

const listeners = {};

export function on(event, callback) {
    if (!listeners[event]) listeners[event] = [];
    listeners[event].push(callback);
}

export function emit(event, data) {
    if (!listeners[event]) return;
    listeners[event].forEach(cb => cb(data));
}
