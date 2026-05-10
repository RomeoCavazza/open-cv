import * as api from '../api.js';
import * as iframeRender from '../render/iframe.js';
import { EVENTS, emit } from '../modules/events.js';
import { selectedLlmProvider, setActiveJobId } from '../state.js';
import * as router from '../router.js';

const MAX_PENDING_BATCHES = 8;
const MAX_PARALLEL_GENERATIONS = 2;

export class IngestController {
    constructor() {
        this.pendingBatches = [];
        this.currentBatch = null;
        this.processing = false;
        this.generationChain = Promise.resolve();
        this.pendingGenerationBatches = 0;
        console.log("[IngestController] Initialized");
    }

    getAllPendingBatches() {
        const all = [...this.pendingBatches];
        if (this.currentBatch) {
            all.unshift(this.currentBatch);
        }
        return all;
    }

    async runIngest() {
        const input = document.getElementById('job-input');
        const rawInput = input?.value?.trim();
        if (!input || !rawInput) return;

        if (this.pendingBatches.length >= MAX_PENDING_BATCHES) {
            emit(EVENTS.NOTIFICATION, {
                message: `File d'attente pleine (${MAX_PENDING_BATCHES}). Patiente quelques secondes.`,
                type: 'error'
            });
            return;
        }

        this.pendingBatches.push({
            input: rawInput,
            provider: selectedLlmProvider,
            options: this.readDeliverables()
        });
        input.value = '';
        if (this.isProcessing() || this.pendingBatches.length > 1) {
            emit(EVENTS.NOTIFICATION, {
                message: "Lien ajouté à la file. Il apparaîtra dans l'INBOX dès que l'ingestion démarre.",
                type: 'info'
            });
        } else {
            emit(EVENTS.NOTIFICATION, {
                message: "Lien envoyé ! Analyse et ingestion en cours...",
                type: 'success'
            });
        }
        emit(EVENTS.GEN_STARTED, {
            pending: this.pendingBatches.length,
            processing: this.processing
        });

        if (!this.processing) {
            await this.processQueue();
        }
    }

    readDeliverables() {
        const delivs = document.getElementById('deliv-selector-ingest');
        return {
            restitution: delivs?.querySelector('[data-deliv="restitution"]')?.classList.contains('active') ?? true,
            resume: delivs?.querySelector('[data-deliv="resume"]')?.classList.contains('active') ?? true,
            cover_letter: delivs?.querySelector('[data-deliv="cover"]')?.classList.contains('active') ?? true,
        };
    }

    normalizeIngestItems(payload) {
        const items = Array.isArray(payload?.items) ? payload.items : [];
        if (items.length > 0) return items;
        if (Array.isArray(payload?.ingested)) {
            return payload.ingested
                .filter(Boolean)
                .map((instanceSlug) => ({ instance_slug: instanceSlug }));
        }
        return [];
    }

    async processQueue() {
        this.processing = true;
        while (this.pendingBatches.length > 0) {
            const batch = this.pendingBatches.shift();
            this.currentBatch = batch;
            emit(EVENTS.GEN_STARTED, { pending: this.getQueueSize(), processing: true });
            try {
                await this.processBatch(batch);
                emit(EVENTS.GEN_COMPLETED, { pending: this.pendingBatches.length });
            } catch (e) {
                emit(EVENTS.GEN_FAILED, { message: e.message });
            } finally {
                this.currentBatch = null;
            }
        }
        this.processing = false;
        emit(EVENTS.GEN_COMPLETED, { pending: 0 });
    }

    async processBatch(batch) {
        const ingestRes = await api.ingestOffer(batch.input);
        const items = this.normalizeIngestItems(ingestRes);
        if (!items.length) throw new Error("Échec ingestion: aucune instance créée");

        const firstOfferSlug = items.find((it) => it.offer_slug)?.offer_slug || ingestRes.job_id;
        if (firstOfferSlug) {
            setActiveJobId(firstOfferSlug);
        }

        emit(EVENTS.OFFER_INGESTED, {
            jobId: firstOfferSlug || null,
            items
        });

        this.enqueueGeneration(items, batch.provider || 'ollama', batch.options);
    }

    enqueueGeneration(items, provider, options) {
        this.pendingGenerationBatches += 1;
        emit(EVENTS.GEN_STARTED, {
            pending: this.getQueueSize(),
            processing: this.isProcessing()
        });

        this.generationChain = this.generationChain
            .catch(() => {})
            .then(() => this.generateForItems(items, provider, options))
            .catch((e) => {
                emit(EVENTS.GEN_FAILED, { message: e?.message || 'Erreur génération' });
            })
            .finally(() => {
                this.pendingGenerationBatches = Math.max(0, this.pendingGenerationBatches - 1);
                emit(EVENTS.GEN_COMPLETED, { pending: this.getQueueSize() });
            });
    }

    async generateForItems(items, provider, options) {
        const queue = items
            .map((item) => ({
                target: item.offer_slug || item.instance_slug,
                storageKey: item.offer_slug || item.instance_slug
            }))
            .filter((item) => !!item.target);

        if (!queue.length) return;

        let cursor = 0;
        const workers = Array.from(
            { length: Math.min(MAX_PARALLEL_GENERATIONS, queue.length) },
            async () => {
                while (cursor < queue.length) {
                    const current = queue[cursor++];
                    await api.generateApplication(
                        current.target,
                        provider,
                        options,
                        current.storageKey
                    );
                }
            }
        );

        await Promise.all(workers);
    }

    getQueueSize() {
        return this.pendingBatches.length
            + (this.processing ? 1 : 0)
            + this.pendingGenerationBatches;
    }

    isProcessing() {
        return this.processing || this.pendingGenerationBatches > 0;
    }

    getQueueLabel() {
        const size = this.getQueueSize();
        if (size <= 1) {
            return this.processing ? 'Generation en cours...' : 'Generate Application';
        }
        return `Queue (${size})`;
    }
}
