export function stringifyDocument(value) {
    if (value == null) return "";
    try {
        return JSON.stringify(value, null, 2);
    } catch (_) {
        return "";
    }
}

export function readFileAsDataUrl(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result || ""));
        reader.onerror = () => reject(new Error('file-read-failed'));
        reader.readAsDataURL(file);
    });
}
