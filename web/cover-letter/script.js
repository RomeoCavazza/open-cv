async function loadCoverLetter() {
  try {
    const urlParams = new URLSearchParams(window.location.search);
    const jobId = urlParams.get('id') || urlParams.get('instance');
    const hasInstance = !!(jobId && jobId !== 'null');

    if (!hasInstance) {
      renderEmptyCoverLetterState(null);
      return;
    }

    const response = await fetch(`/api/instances/${jobId}/cover-letter`);

    if (!response.ok) {
      renderEmptyCoverLetterState(jobId);
      return;
    }

    const data = await response.json();
    renderTemplateCoverLetter(data);
  } catch (error) {
    console.error("Unable to load cover letter data:", error);
  }
}

function renderEmptyCoverLetterState(jobId) {
  const stage = document.querySelector('.page-stage');
  if (!stage) return;

  const hasGenerateAction = !!jobId;
  stage.innerHTML = `
    <div style="display: flex; flex-direction: column; align-items: center; justify-content: flex-start; height: 100vh; width: 100%; padding-top: 18vh; padding-left: 40px; padding-right: 40px; text-align: center; color: #64748b; background: #fff;">
        <div style="width: 64px; height: 64px; background: #eff6ff; border-radius: 50%; display: flex; align-items: center; justify-content: center; margin-bottom: 24px; color: #0052ff;">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" style="width: 32px; height: 32px;">
              <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
            </svg>
        </div>
        <h2 style="font-size: 20px; font-weight: 700; color: #1e293b; margin-bottom: 12px;">Lettre non disponible</h2>
        ${hasGenerateAction ? `<button id="btn-generate-cover" style="
            background: #0052ff;
            color: white;
            border: none;
            padding: 14px 32px;
            border-radius: 12px;
            font-weight: 600;
            cursor: pointer;
            font-size: 15px;
            box-shadow: 0 4px 12px rgba(0,82,255,0.2);
            transition: all 0.2s;
        ">Générer la lettre</button>` : ''}
    </div>
  `;
  if (window.lucide) lucide.createIcons();
  // No applyPreviewScale here as we want full-width/height

  if (hasGenerateAction) {
    document.getElementById('btn-generate-cover').onclick = async () => {
      const btn = document.getElementById('btn-generate-cover');
      btn.disabled = true;
      btn.innerText = "Génération...";
      try {
        const res = await fetch(`/api/instances/${jobId}/generate`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ deliverables: { cover: true, resume: false, restitution: false } })
        });
        if (res.ok) window.location.reload();
        else btn.disabled = false;
      } catch (e) { btn.disabled = false; }
    };
  }
}

function renderTemplateCoverLetter(data) {
  const { profile, letter } = data;

  setText("author-name", profile.name);
  setText("author-address", profile.address);
  setText("author-phone", profile.phone);
  setText("author-email", profile.email);

  setLink("author-linkedin", profile.links?.linkedin);
  setLink("author-github", profile.links?.github);
  setLink("author-website", profile.links?.website);

  // Set Labels
  if (letter.labels) {
    setText("author-linkedin", letter.labels.linkedin);
    setText("author-github", letter.labels.github);
    setText("author-website", letter.labels.website);

    const downloadBtn = document.getElementById("download-pdf");
    if (downloadBtn) {
      const span = downloadBtn.querySelector("span") || downloadBtn;
      // If we want to preserve icons, we should be careful. 
      // But index.html current structure doesn't have a span.
      // Let's just update the text and preserve icon if we find it.
      const icon = downloadBtn.querySelector("i");
      downloadBtn.textContent = "";
      if (icon) downloadBtn.appendChild(icon);
      downloadBtn.appendChild(document.createTextNode(" " + letter.labels.download));
    }
  }

  setText("letter-company", letter.company);
  setText("letter-date", letter.date);

  // Bold only the specific keyword part of the subject
  const boldKeyword = letter.boldKeyword || "ALTERNANCE";
  const subjectRegex = new RegExp(`^\\s*(${boldKeyword})`, 'i');
  const subjectContent = letter.subject.replace(subjectRegex, '<strong>$1</strong>');
  setText("letter-subject", subjectContent, true);

  setText("letter-greeting", letter.greeting);
  setPitchBlock(letter);

  const paragraphsContainer = document.getElementById("letter-paragraphs");
  if (paragraphsContainer) {
    paragraphsContainer.innerHTML = "";
    (letter.paragraphs || []).forEach((text) => {
      const paragraph = document.createElement("p");
      paragraph.className = "paragraph";
      paragraph.textContent = text;
      paragraphsContainer.appendChild(paragraph);
    });
  }

  setText("letter-closing", letter.closing);

  const signatureEl = document.getElementById("letter-signature");
  if (signatureEl && letter.signature) {
    signatureEl.innerHTML = letter.signature
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean)
      .join("<br>");
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

  el.innerHTML = "";
  combined.forEach((line, index) => {
    if (index > 0) {
      el.appendChild(document.createElement("br"));
    }
    const span = document.createElement("span");
    span.className = "pitch-line";

    // Bold labels (text before :)
    if (line.includes(" :")) {
      const parts = line.split(" :");
      span.innerHTML = `<strong>${parts[0]} :</strong>${parts[1]}`;
    } else {
      span.textContent = line;
    }

    el.appendChild(span);
  });
}

function setText(id, value, html = false) {
  const el = document.getElementById(id);
  if (el && value) {
    if (html) {
      el.innerHTML = value;
    } else {
      el.textContent = value;
    }
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
    const availableWidth = window.innerWidth - 32;
    const availableHeight = window.innerHeight - 32;
    const pageWidth = page.offsetWidth;
    const pageHeight = page.offsetHeight;
    const scale = Math.min(1, availableWidth / pageWidth, availableHeight / pageHeight);
    const scaledWidth = pageWidth * scale;
    const scaledHeight = pageHeight * scale;
    const offsetX = Math.max(0, (availableWidth - scaledWidth) / 2);
    const offsetY = 16;

    page.style.transform = `scale(${scale})`;
    page.style.left = `${offsetX}px`;
    page.style.top = `${offsetY}px`;
    stage.style.height = `${window.innerHeight}px`;
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
