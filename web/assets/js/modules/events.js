console.log("[EventBus] Module Loading...");
/**
 * RecruitAI - Unified Event Bus
 * Centralizes application events to decouple frontend modules.
 * Based on native EventTarget for performance and reliability.
 */

export const EVENTS = {
    // Offer Events
    OFFER_SELECTED: 'OFFER_SELECTED',
    OFFER_INGESTED: 'INGEST_COMPLETED', // Aliased for chat.js compatibility
    OFFER_DELETED: 'OFFER_DELETED',
    
    // Generation Events
    GEN_STARTED: 'GEN_STARTED',
    GEN_STEP: 'GEN_STEP',
    GEN_COMPLETED: 'GEN_COMPLETED',
    GEN_FAILED: 'GEN_FAILED',
    
    // UI & State Events
    PROFILE_UPDATED: 'PROFILE_UPDATED',
    LLM_PROVIDER_CHANGED: 'LLM_PROVIDER_CHANGED',
    DELIV_CONFIG_CHANGED: 'DELIV_CONFIG_CHANGED',
    NAVIGATE: 'NAVIGATE',
    NOTIFICATION: 'NOTIFICATION',
    UPDATE_IFRAME: 'UPDATE_IFRAME'
};

const bus = new EventTarget();

/**
 * Subscribe to an event
 * @param {string} eventName 
 * @param {Function} callback 
 */
export function on(eventName, callback) {
    const handler = (e) => callback(e.detail);
    bus.addEventListener(eventName, handler);
    return handler;
}

/**
 * Dispatch an event with a custom payload
 * @param {string} eventName 
 * @param {Object} detail 
 */
export function emit(eventName, detail = {}) {
    const event = new CustomEvent(eventName, { detail });
    bus.dispatchEvent(event);
    console.debug(`[EventBus] ${eventName}`, detail);
}

// Legacy global expose for dashboard.js transition
window.AppEvents = { EVENTS, on, emit };
