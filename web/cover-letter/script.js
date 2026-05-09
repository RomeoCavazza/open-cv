import { clear, el, svg, text } from '../assets/js/dom.js';
import { playSuccessSound } from '../assets/js/render/audio.js';

// Local flag removed in favor of localStorage
window.handleGenerate = async () => {
  const urlParams = new URLSearchParams(window.location.search);
  let jobId = urlParams.get('id') || urlParams.get('instance');
  const offerId = urlParams.get('offer');
  if (!jobId) return;

  const provider = window.parent.state?.selectedLlmProvider || 'claude';

  try {
    const result = await window.parent.api.generateApplication(jobId, provider, { cover_letter: true }, offerId);
    if (result && result.slug) {
        window._currentJobId = result.slug;
    }
    showGenerating();
    if (!window.pollInterval) window.pollInterval = setInterval(loadCoverLetter, 2000);
  } catch (e) {
    console.error(e);
    loadCoverLetter();
  }
};

function showGenerating() {
  const gen = document.getElementById('generating-state');
  const con = document.getElementById('cl-container');
  const emp = document.getElementById('empty-state');
  if (gen) gen.style.display = 'flex';
  if (con) con.style.display = 'none';
  if (emp) emp.style.display = 'none';
  if (window.lucide) lucide.createIcons();
}

function showContent() {
  const gen = document.getElementById('generating-state');
  const con = document.getElementById('cl-container');
  const emp = document.getElementById('empty-state');
  if (gen) gen.style.display = 'none';
  if (con) con.style.display = 'block';
  if (emp) emp.style.display = 'none';
}

async function loadCoverLetter() {
  const urlParams = new URLSearchParams(window.location.search);
  const jobIdInUrl = urlParams.get('id') || urlParams.get('instance');
  const offerId = urlParams.get('offer');
  
  const jobId = window._currentJobId || jobIdInUrl;
  if (!jobId) return;

  try {
    const hasInstance = !!(jobId && jobId !== 'null');
    if (!hasInstance) return renderEmptyCoverLetterState(null);

    let resInstance = await fetch(`/api/instances/${jobId}?t=${Date.now()}`);
    if (!resInstance.ok && !jobId.includes('__')) {
        resInstance = await fetch(`/api/offres/${jobId}/instance?t=${Date.now()}`);
    }

    if (!resInstance.ok) return renderEmptyCoverLetterState(jobId);
    const instance = await resInstance.json();
    const status = instance.status.toLowerCase();

    let genTarget = { cover_letter: false }; 
    const storageKey = offerId || jobIdInUrl;
    try {
        const stored = localStorage.getItem('generating_target_' + storageKey);
        if (stored) genTarget = JSON.parse(stored);
    } catch(e) {}

    if (instance.cover_letter_json) {
      if (genTarget.cover_letter) {
        genTarget.cover_letter = false;
        localStorage.setItem('generating_target_' + storageKey, JSON.stringify(genTarget));
      }
      showContent();
      renderTemplateCoverLetter(instance.cover_letter_json);
      applyPreviewScale();
    } else if (genTarget.cover_letter) {
      if (status === 'failed') {
        genTarget.cover_letter = false;
        localStorage.setItem('generating_target_' + storageKey, JSON.stringify(genTarget));
        renderEmptyCoverLetterState(jobId, false);
      } else {
        showGenerating();
      }
    } else {
      renderEmptyCoverLetterState(jobId, false);
    }
  } catch (error) {
    console.error("Unable to load cover letter:", error);
  }
}

// Reactive UI updates
window.addEventListener('storage', (e) => {
    const urlParams = new URLSearchParams(window.location.search);
    const storageKey = urlParams.get('offer') || urlParams.get('id') || urlParams.get('instance');
    if (e.key === 'generating_target_' + storageKey) {
        loadCoverLetter();
    }
});

function renderEmptyCoverLetterState(jobId, isInstanceGenerating = false) {
  const stage = document.getElementById('empty-state');
  const gen = document.getElementById('generating-state');
  const con = document.getElementById('cl-container');
  if (gen) gen.style.display = 'none';
  if (con) con.style.display = 'none';
  if (!stage) return;
  stage.style.display = 'flex';

  const hasGenerateAction = !!jobId;

  clear(stage).appendChild(el('div', {
    style: 'display:flex; flex-direction:column; align-items:center;'
  }, [
    el('div', {
      style: 'width:64px; height:64px; background:#eff6ff; border-radius:50%; display:flex; align-items:center; justify-content:center; margin-bottom:24px; color:#0052ff;'
    }, [
      svg('svg', {
        xmlns: 'http://www.w3.org/2000/svg',
        fill: 'none',
        viewBox: '0 0 24 24',
        'stroke-width': '1.5',
        stroke: 'currentColor',
        style: 'width:32px; height:32px;'
      }, [svg('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        d: 'M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z'
      })])
    ]),
    el('h2', {
      style: 'font-size:20px; font-weight:700; color:#1e293b; margin-bottom:12px;',
      text: 'Lettre non disponible'
    }),
    hasGenerateAction ? el('button', {
      id: 'btn-generate-cover',
      style: 'background:#0052ff; color:white; border:none; padding:14px 32px; border-radius:12px; font-weight:600; cursor:pointer; font-size:15px; box-shadow:0 4px 12px rgba(0,82,255,0.2); transition:all 0.2s;',
      text: 'Générer la lettre',
      onclick: () => window.handleGenerate()
    }) : null
  ]));
  if (window.lucide) lucide.createIcons();
}

function renderTemplateCoverLetter(data) {
  setText("author-name", data.expediteur.identite.nom_complet);
  setText("author-address", data.expediteur.contact.localisation);
  setText("author-phone", data.expediteur.contact.telephone);
  setText("author-email", data.expediteur.contact.email);

  setLink("author-linkedin", data.expediteur.contact.linkedin);
  setLink("author-github", data.expediteur.contact.github);
  setLink("author-website", data.expediteur.contact.site_web);

  setText("letter-company", data.destinataire.entreprise);
  setText("letter-date", data.destinataire.date);

  const subjectEl = document.getElementById("letter-subject");
  if (subjectEl) {
    subjectEl.replaceChildren();
    const strong = document.createElement('strong');
    strong.textContent = data.objet.categorie + " — ";
    subjectEl.appendChild(strong);
    subjectEl.appendChild(document.createTextNode(data.objet.libelle));
  }

  const salutationPara = data.paragraphes.find(p => p.role === 'salutation');
  setText("letter-greeting", salutationPara ? salutationPara.contenu : "");

  const paragraphsContainer = document.getElementById("letter-paragraphs");
  if (paragraphsContainer) {
    clear(paragraphsContainer);
    data.paragraphes
      .filter(p => !['salutation', 'cloture'].includes(p.role))
      .forEach((p) => {
        paragraphsContainer.appendChild(el('p', { className: 'paragraph', text: p.contenu }));
      });
  }

  const cloturePara = data.paragraphes.find(p => p.role === 'cloture');
  setText("letter-closing", cloturePara ? cloturePara.contenu : "");

  const signatureEl = document.getElementById("letter-signature");
  if (signatureEl) {
    signatureEl.replaceChildren();
    signatureEl.appendChild(document.createTextNode(data.signature.formule_politesse));
    signatureEl.appendChild(document.createElement('br'));
    signatureEl.appendChild(document.createTextNode(data.signature.nom));
  }

  if (window.lucide?.createIcons) {
    window.lucide.createIcons();
  }
}

function setPitchBlock(letter) {
  const el = document.getElementById("letter-pitch");
  if (!el) return;

  const collectLines = (input, target) => {
    if (Array.isArray(input)) {
      input.forEach((item) => collectLines(item, target));
      return;
    }
    if (input == null) return;
    const trimmed = String(input).trim();
    if (trimmed) {
      target.push(trimmed);
    }
  };

  const pitchLines = [];
  const metadataLines = [];

  collectLines(letter.pitch, pitchLines);
  collectLines(letter.metadata, metadataLines);

  const combined = pitchLines.concat(metadataLines);
  if (!combined.length) {
    el.remove();
    return;
  }

  clear(el);
  combined.forEach((line, index) => {
    if (index > 0) {
      el.appendChild(document.createElement("br"));
    }
    const span = document.createElement("span");
    span.className = "pitch-line";

    // Bold labels (text before :)
    if (line.includes(" :")) {
      const parts = line.split(" :");
      const strong = document.createElement('strong');
      strong.textContent = `${parts[0]} :`;
      span.appendChild(strong);
      span.appendChild(document.createTextNode(parts.slice(1).join(' :')));
    } else {
      span.textContent = line;
    }

    el.appendChild(span);
  });
}

function setText(id, value) {
  const el = document.getElementById(id);
  if (el && value) {
    el.textContent = value;
  }
}

function setLink(id, value) {
  const el = document.getElementById(id);
  if (!el || !value) return;

  const href = /^https?:\/\//i.test(value) ? value : `https://${value}`;
  el.href = href;
}

function applyPreviewScale() {
  if (window.self === window.top) return;

  const page = document.querySelector(".a4-page");
  const stage = document.querySelector(".page-stage");
  if (!page || !stage) return;

  page.style.transform = "none";
  page.style.left = "0px";
  page.style.top = "0px";

  requestAnimationFrame(() => {
    const availableWidth = window.innerWidth - 40;
    const availableHeight = window.innerHeight - 40;
    const pageWidth = page.offsetWidth;
    const pageHeight = page.offsetHeight;
    
    // Calcul identique pour un rendu uniforme
    const scale = Math.min(1, availableWidth / pageWidth, availableHeight / pageHeight);
    const scaledWidth = pageWidth * scale;
    const scaledHeight = pageHeight * scale;
    
    const offsetX = Math.max(0, (availableWidth - scaledWidth) / 2) + 20;
    const offsetY = 20;

    page.style.transformOrigin = "top center";
    page.style.transform = `translateX(-50%) scale(${scale})`;
    page.style.left = "50%";
    page.style.top = `${offsetY}px`;
    if (stage) stage.style.height = `${window.innerHeight}px`;
  });
}

const downloadButton = document.getElementById("download-pdf");
if (downloadButton) {
  downloadButton.addEventListener("click", () => window.print());
}

window.addEventListener('DOMContentLoaded', () => {
  if (window.self !== window.top) {
    document.body.classList.add("is-framed");
  }
  loadCoverLetter();
  applyPreviewScale();
});
window.addEventListener("resize", applyPreviewScale);
