import { selectedLlmProvider, delivConfig, setDelivConfig, setSelectedLlmProvider } from '../state.js';
import { EVENTS, emit } from '../modules/events.js';

export function setupSelector(containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    container.querySelectorAll('.llm-pill').forEach(pill => {
        const prov = pill.dataset.provider;
        const deliv = pill.dataset.deliv;

        if (prov) {
            if (selectedLlmProvider === prov) pill.classList.add('active');
            else pill.classList.remove('active');
        } else if (deliv) {
            const val = delivConfig[deliv];
            if (val === true) pill.classList.add('active');
            else if (val === false) pill.classList.remove('active');
        }

        pill.onclick = (e) => {
            e.preventDefault();
            if (prov) {
                setSelectedLlmProvider(prov);
                emit(EVENTS.LLM_PROVIDER_CHANGED, { provider: prov });
            } else if (deliv) {
                const newVal = !delivConfig[deliv];
                delivConfig[deliv] = newVal;
                setDelivConfig({ ...delivConfig });
                if (newVal) pill.classList.add('active');
                else pill.classList.remove('active');
            }
        };
    });
}
