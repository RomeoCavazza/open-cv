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
                const jobId = data.jobId;
                const key = 'generating_target_' + jobId;
                
                if (!this.activeJobs.has(jobId)) {
                    console.log(`[BackgroundPollManager] Starting immediate poll for ${jobId}`);
                    this.startPolling(jobId, key);
                } else {
                    const state = this.activeJobs.get(jobId);
                    console.log(`[BackgroundPollManager] Resetting state for ${jobId} (New Gen started)`);
                    state.hasSeenActiveState = false;
                }
            } else {
                this.scan();
            }
        });

        // 4. Safety net
        setInterval(() => this.scan(), 10000);
    }

    scan() {
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i);
            if (key && key.startsWith('generating_target_')) {
                const jobId = key.replace('generating_target_', '');
                try {
                    const target = JSON.parse(localStorage.getItem(key));
                    const lastTriggered = target.last_triggered || 0;
                    
                    // Expiration safety: if a flag is older than 30 minutes, it's a ghost.
                    if (Date.now() - lastTriggered > 30 * 60 * 1000) {
                        console.warn(`[BackgroundPollManager] Expired flag for ${jobId}. Cleaning up.`);
                        localStorage.removeItem(key);
                        continue;
                    }

                    const isGenerating = Object.values(target).some(v => v === true && v !== target.last_triggered);
                    
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

        let retryCount = 0;
        const poll = async () => {
            if (controller.signal.aborted) return;

            try {
                let instance = null;
                let res = await fetch(`/api/instances/${jobId}?t=${Date.now()}`, { signal: controller.signal });
                
                if (res.ok) {
                    instance = await res.json();
                } else if (!jobId.includes('__')) {
                    // Try by offer slug
                    const resOffre = await fetch(`/api/offres/${jobId}/instance?t=${Date.now()}`, { signal: controller.signal });
                    if (resOffre.ok) {
                        instance = await resOffre.json();
                    }
                }

                if (!instance) {
                    retryCount++;
                    if (retryCount > 3) {
                        console.warn(`[BackgroundPollManager] ${jobId} not found. Cleaning up stale flag.`);
                        localStorage.removeItem(storageKey);
                        this.stopPolling(jobId);
                        return;
                    }
                    setTimeout(poll, 3000);
                    return;
                }

                retryCount = 0; // Reset on success
                
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
        // The backend uses 'ready' for success, 'failed' for failure.
        const isBackendDone = status === 'ready' || status === 'failed';
        const stillWaitingInUi = Object.values(target).some(v => v === true && v !== target.last_triggered);

        // Versioning check: Is the data in the API fresh or from a previous run?
        const lastTriggered = target.last_triggered || 0;
        const updatedAt = new Date(instance.updated_at).getTime();
        const isDataFresh = updatedAt >= (lastTriggered - 2000); // 2s margin for clock skew

        console.log(`[BackgroundPollManager] ${jobId} Status: ${status}, Fresh: ${isDataFresh}, ActiveSeen: ${jobState.hasSeenActiveState}`);

        // Race condition protection: If backend says done but it's old data, ignore it.
        if (isBackendDone && !isDataFresh && stillWaitingInUi) {
            console.log(`[BackgroundPollManager] ${jobId}: Ignoring stale backend state.`);
            setTimeout(next, 2000);
            return;
        }

        // If we see an active or fresh state, we mark the job as "seen active" for this session
        if (!isBackendDone || isDataFresh) {
            jobState.hasSeenActiveState = true;
        }

        let changed = false;
        
        // Sync targets ONLY if data is fresh
        if (isDataFresh) {
            if (target.restitution && instance.restitution) { target.restitution = false; changed = true; }
            if (target.resume && instance.resume_json) { target.resume = false; changed = true; }
            if (target.cover_letter && instance.cover_letter_json) { target.cover_letter = false; changed = true; }
        }

        // Handle termination
        if (isBackendDone && isDataFresh) {
            // Force clear if backend is done for this run
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
            // Only play sound if it was a SUCCESS and we actually saw it happening
            if (status === 'ready' && jobState.hasSeenActiveState) {
                console.log(`[BackgroundPollManager] ${jobId} SUCCESS (ready). Playing sound.`);
                playSuccessSound();
            } else if (status === 'failed') {
                console.log(`[BackgroundPollManager] ${jobId} FAILED. No sound.`);
            }
            this.stopPolling(jobId);
            emit(EVENTS.UPDATE_IFRAME); // Final refresh
        } else {
            setTimeout(next, 2000);
        }
    }
}

export const backgroundPollManager = new BackgroundPollManager();

