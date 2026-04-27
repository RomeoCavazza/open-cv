(() => {
  const API = 'task06.php';
  const POLL = 4000;
  let lastId = 0;

  const chat = document.getElementById('chat');
  const name = document.getElementById('name');
  const text = document.getElementById('text');
  const send = document.getElementById('send');
  const reset = document.getElementById('reset');

  function append(messages) {
    for (const m of messages) {
      lastId = Math.max(lastId, Number(m.id));
      const row = document.createElement('div');
      const mine = (m.name.toLowerCase() === (name.value.trim().toLowerCase() || 'tco'));
      row.className = 'msg ' + (mine ? 'me' : 'other');
      const avatar = document.createElement('div');
      avatar.className = 'avatar';
      avatar.textContent = (m.name[0] || '?').toUpperCase();
      const body = document.createElement('div');
      const bubble = document.createElement('div');
      bubble.className = 'bubble';
      bubble.textContent = m.text;
      const meta = document.createElement('div');
      meta.className = 'meta';
      meta.textContent = `${m.name} · ${m.ts}`;
      body.appendChild(bubble);
      body.appendChild(meta);
      row.appendChild(avatar);
      row.appendChild(body);
      chat.appendChild(row);
    }
    if (messages.length) chat.scrollTop = chat.scrollHeight;
  }

  async function load() {
    try {
      const url = new URL(API, location);
      url.search = new URLSearchParams({ action: 'list', since_id: String(lastId) }).toString();
      const r = await fetch(url, { cache: 'no-store' });
      const data = await r.json();
      append(data.messages || []);
    } catch {}
  }

  async function post() {
    const n = name.value.trim() || 'tco';
    const t = text.value.trim();
    if (!t) return;
    send.disabled = true;
    try {
      const url = new URL(API, location);
      url.search = new URLSearchParams({ action: 'post', name: n, text: t }).toString();
      await fetch(url);
      text.value = '';
      await load();
    } finally {
      send.disabled = false;
      text.focus();
    }
  }

  async function wipe() {
    if (!confirm('Vider les messages ?')) return;
    await fetch(`${API}?action=reset`);
    chat.innerHTML = '';
    lastId = 0;
  }

  send.addEventListener('click', post);
  text.addEventListener('keydown', e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); post(); } });
  reset.addEventListener('click', wipe);

  load();
  setInterval(load, POLL);
  text.focus();
})();
