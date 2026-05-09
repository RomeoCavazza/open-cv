import { EVENTS, emit } from './events.js';
import { playSuccessSound } from '../render/audio.js';

/**
 * BackgroundPollManager
 * Reactive manager that monitors generation state via 'storage' events.
 * It centralizes all API polling to avoid redundant requests from multiple iframes.
 */
class BackgroundPollManager {
    constructor() {
        this.activeJobs = new Map(); // jobId -> abortController
        console.log("[BackgroundPollManager] Reactive Initialized");
    }

    start() {
        // 1. Initial scan on load
        this.scan();

        // 2. Listen for storage changes (triggered by iframes or other tabs)
        window.addEventListener('storage', (e) => {
            if (e.key && e.key.startsWith('generating_target_')) {
                this.scan();
            }
        });

        // 3. Optional: small periodic scan as a safety net (heartbeat)
        setInterval(() => this.scan(), 10000);
    }

    scan() {
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i);
            if (key.startsWith('generating_target_')) {
                const jobId = key.replace('generating_target_', '');
                try {
                    const target = JSON.parse(localStorage.getItem(key));
                    const isGenerating = Object.values(target).some(v => v === true);
                    
                    if (isGenerating && !this.activeJobs.has(jobId)) {
                        this.startPolling(jobId, key);
                    } else if (!isGenerating && this.activeJobs.has(jobId)) {
                        this.stopPolling(jobId);
                    }
                } catch (e) {}
            }
        }
    }

    stopPolling(jobId) {
        const controller = this.activeJobs.get(jobId);
        if (controller) {
            controller.abort();
            this.activeJobs.delete(jobId);
            console.log(`[BackgroundPollManager] Stopped polling for ${jobId}`);
        }
    }

    async startPolling(jobId, storageKey) {
        const controller = new AbortController();
        this.activeJobs.set(jobId, controller);
        console.log(`[BackgroundPollManager] Started centralized polling for: ${jobId}`);

        const poll = async () => {
            if (controller.signal.aborted) return;

            try {
                const stored = localStorage.getItem(storageKey);
                if (!stored) return this.stopPolling(jobId);
                
                const target = JSON.parse(stored);
                if (!Object.values(target).some(v => v === true)) return this.stopPolling(jobId);

                // API Request
                const res = await fetch(`/api/instances/${jobId}?t=${Date.now()}`, { signal: controller.signal });
                let instance;
                
                if (!res.ok) {
                    const resOffre = await fetch(`/api/offres/${jobId}/instance?t=${Date.now()}`, { signal: controller.signal });
                    if (!resOffre.ok) {
                        return setTimeout(poll, 2500);
                    }
                    instance = await resOffre.json();
                } else {
                    instance = await res.json();
                }

                this.processUpdate(instance, target, storageKey, jobId, poll);
            } catch (e) {
                if (e.name === 'AbortError') return;
                setTimeout(poll, 3000);
            }
        };

        poll();
    }

    processUpdate(instance, target, storageKey, jobId, next) {
        const status = instance.status.toLowerCase();
        let completedSomething = false;

        if (target.restitution && instance.restitution) { target.restitution = false; completedSomething = true; }
        if (target.resume && instance.resume_json) { target.resume = false; completedSomething = true; }
        if (target.cover_letter && instance.cover_letter_json) { target.cover_letter = false; completedSomething = true; }

        if (status === 'failed') {
            Object.keys(target).forEach(k => target[k] = false);
            completedSomething = true;
        }

        if (completedSomething) {
            localStorage.setItem(storageKey, JSON.stringify(target));
            
            // Completion Logic
            const isFullyDone = !Object.values(target).some(v => v === true);
            if (isFullyDone || completedSomething) {
                playSuccessSound();
                emit(EVENTS.UPDATE_IFRAME); 
            }
        }

        if (Object.values(target).some(v => v === true)) {
            setTimeout(next, 2000);
        } else {
            this.stopPolling(jobId);
        }
    }
}

export const backgroundPollManager = new BackgroundPollManager();

