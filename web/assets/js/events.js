/**
 * RecruitAI - Event Bus
 * Centralizes application events to decouple frontend modules.
 * Based on native EventTarget for maximum performance and zero dependencies.
 */

export const AppEvents = {
    // Offer Events
    OFFER_SELECTED: 'offer:selected',
    OFFER_INGESTED: 'offer:ingested',
    OFFER_DELETED: 'offer:deleted',
    
    // Generation Events
    GEN_STARTED: 'gen:started',
    GEN_STEP: 'gen:step',
    GEN_COMPLETED: 'gen:completed',
    GEN_FAILED: 'gen:failed',
    
    // UI & State Events
    PROFILE_UPDATED: 'profile:updated',
    NAVIGATE: 'ui:navigate',
    NOTIFICATION: 'ui:notification',
    
    /**
     * Dispatch an event with a custom payload
     * @param {string} eventName 
     * @param {Object} detail 
     */
    emit(eventName, detail = {}) {
        const event = new CustomEvent(eventName, { detail });
        window.dispatchEvent(event);
        console.debug(`[EventBus] Emitted: ${eventName}`, detail);
    },

    /**
     * Subscribe to an event
     * @param {string} eventName 
     * @param {Function} callback 
     */
    on(eventName, callback) {
        window.addEventListener(eventName, (e) => callback(e.detail));
    },

    /**
     * Unsubscribe from an event
     * @param {string} eventName 
     * @param {Function} callback 
     */
    off(eventName, callback) {
        window.removeEventListener(eventName, callback);
    }
};

// Also expose as global for legacy dashboard.js compatibility during transition
window.AppEvents = AppEvents;
