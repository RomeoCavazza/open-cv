async function refreshCurrentTab() {
    if (!state.activeJobId) {
        resetIframeToEmptyState();
        return;
    }

    if (state.activeTab === 'restitution' && window.activeInstanceSlug) {
        try {
            fetch(
                \`/api/instances/\${encodeURIComponent(window.activeInstanceSlug)}/generate?restitution=true&resume=false&cover_letter=false&llm_provider=\${encodeURIComponent(state.selectedLlmProvider)}\`,
                { method: 'POST' }
            );
            await updateIframe();
            return;
        } catch (error) {
            console.warn('Impossible de forcer la régénération', error);
        }
    }

    await updateIframe();
}
