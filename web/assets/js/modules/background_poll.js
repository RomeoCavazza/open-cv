import { EVENTS, emit, on } from './events.js';
import { playSuccessSound } from '../render/audio.js';

/**
 * BackgroundPollManager
 * Reactive manager that monitors generation state via 'storage' events.
 * It centralizes all API polling to avoid redundant requests from multiple iframes.
 */
class BackgroundPollManager {
    constructor() {
        this.activeJobs = new Map(); // jobId -> { controller, hasSeenActiveState }
        console.log("[BackgroundPollManager] Reactive Initialized");
    }

    start() {
        // 1. Initial scan on load
        this.scan();

        // 2. Listen for storage changes
        window.addEventListener('storage', (e) => {
            if (e.key && e.key.startsWith('generating_target_')) {
                this.scan();
            }
        });

        // 3. Listen for local events
        on(EVENTS.GEN_STARTED, (data) => {
            if (data?.jobId) {
                const state = this.activeJobs.get(data.jobId);
                if (state) {
                    console.log(`[BackgroundPollManager] Resetting state for ${data.jobId} (New Gen started)`);
                    state.hasSeenActiveState = false;
                }
            }
            this.scan();
        });

        // 4. Safety net
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
        const state = this.activeJobs.get(jobId);
        if (state) {
            state.controller.abort();
            this.activeJobs.delete(jobId);
            console.log(`[BackgroundPollManager] Stopped polling for ${jobId}`);
        }
    }

    async startPolling(jobId, storageKey) {
        const controller = new AbortController();
        const jobState = {
            controller,
            hasSeenActiveState: false
        };
        this.activeJobs.set(jobId, jobState);
        console.log(`[BackgroundPollManager] Started centralized polling for: ${jobId}`);

        const poll = async () => {
            if (controller.signal.aborted) return;

            try {
                const stored = localStorage.getItem(storageKey);
                if (!stored) return this.stopPolling(jobId);
                
                const target = JSON.parse(stored);
                const stillGeneratingInUi = Object.values(target).some(v => v === true);
                if (!stillGeneratingInUi) return this.stopPolling(jobId);

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

                // Check for active state in API response
                const status = instance.status.toLowerCase();
                if (status === 'generating' || status === 'pending') {
                    jobState.hasSeenActiveState = true;
                }

                this.processUpdate(instance, target, storageKey, jobId, poll, jobState);
            } catch (e) {
                if (e.name === 'AbortError') return;
                setTimeout(poll, 3000);
            }
        };

        poll();
    }

    processUpdate(instance, target, storageKey, jobId, next, jobState) {
        const status = instance.status.toLowerCase();
        const isBackendDone = status === 'finished' || status === 'failed';
        const stillWaitingInUi = Object.values(target).some(v => v === true && v !== target.last_triggered);

        // Versioning check: Is the data in the API fresh or from a previous run?
        const lastTriggered = target.last_triggered || 0;
        const updatedAt = new Date(instance.updated_at).getTime();
        const isDataFresh = updatedAt >= (lastTriggered - 2000); // 2s margin for clock skew

        // Race condition protection: If backend says 'finished' but we haven't seen it transition 
        // through 'pending'/'generating' yet, it's likely the OLD state from a previous run.
        if (isBackendDone && !jobState.hasSeenActiveState && stillWaitingInUi) {
            console.log(`[BackgroundPollManager] Polling ${jobId}: Ignoring stale finished state.`);
            setTimeout(next, 2000);
            return;
        }

        let changed = false;
        
        // Sync targets ONLY if data is fresh or backend is officially finished
        if (isDataFresh || isBackendDone) {
            if (target.restitution && instance.restitution) { target.restitution = false; changed = true; }
            if (target.resume && instance.resume_json) { target.resume = false; changed = true; }
            if (target.cover_letter && instance.cover_letter_json) { target.cover_letter = false; changed = true; }
        } else {
            console.log(`[BackgroundPollManager] Polling ${jobId}: Data is stale (last_triggered: ${lastTriggered}, updated_at: ${updatedAt}). Keeping skeletons.`);
        }

        if (status === 'failed' || (status === 'finished' && stillWaitingInUi)) {
            // Force clear if backend is done
            Object.keys(target).forEach(k => {
                if (k !== 'last_triggered') target[k] = false;
            });
            changed = true;
        }

        if (changed) {
            localStorage.setItem(storageKey, JSON.stringify(target));
            emit(EVENTS.UPDATE_IFRAME); 
        }

        const isFullyDoneNow = !Object.entries(target).some(([k, v]) => k !== 'last_triggered' && v === true);

        if (isFullyDoneNow) {
            if (jobState.hasSeenActiveState) {
                console.log(`[BackgroundPollManager] Job ${jobId} completed. Playing sound.`);
                playSuccessSound();
            }
            this.stopPolling(jobId);
            emit(EVENTS.UPDATE_IFRAME); // Final refresh
        } else {
            setTimeout(next, 2000);
        }
    }
}

export const backgroundPollManager = new BackgroundPollManager();

