import * as api from '../api.js';
import { EVENTS, emit, on } from '../modules/events.js';
import { 
    activeJobId, 
    selectedLlmProvider, 
    delivConfig, 
    setActiveJobId 
} from '../state.js';
import * as router from '../router.js';

export class IngestController {
    constructor() {
        console.log("[IngestController] Initialized");
    }

    async runIngest() {
        const input = document.getElementById('job-input');
        if (!input || !input.value.trim()) return;

        emit(EVENTS.GEN_STARTED);

        try {
            const ingestRes = await api.ingestOffer(input.value);
            if (!ingestRes.job_id) throw new Error("Échec ingestion");

            emit(EVENTS.OFFER_INGESTED, { jobId: ingestRes.job_id });
            setActiveJobId(ingestRes.job_id);

            const delivs = document.getElementById('deliv-selector-ingest');
            const options = {
                restitution: delivs?.querySelector('[data-deliv="restitution"]')?.classList.contains('active') ?? true,
                resume: delivs?.querySelector('[data-deliv="resume"]')?.classList.contains('active') ?? true,
                cover_letter: delivs?.querySelector('[data-deliv="cover"]')?.classList.contains('active') ?? true,
            };

            await api.generateApplication(ingestRes.job_id, selectedLlmProvider, options);
            emit(EVENTS.GEN_COMPLETED, { jobId: ingestRes.job_id });
            emit(EVENTS.NOTIFICATION, { message: 'Application générée avec succès !', type: 'success' });

        } catch (e) {
            emit(EVENTS.GEN_FAILED, { message: e.message });
            emit(EVENTS.NOTIFICATION, { message: 'Erreur: ' + e.message, type: 'error' });
        }
    }
}
