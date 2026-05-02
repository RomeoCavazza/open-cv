async function loadCoverLetter() {
  try {
    const urlParams = new URLSearchParams(window.location.search);
    const jobId = urlParams.get('id');
    const t = Date.now();
    let dataPath = (jobId && jobId !== 'null') ? `/api/instances/${jobId}/cover-letter` : `/api/profile/active/cover-letter-template?t=${t}`;
    
    let response = await fetch(dataPath);
    let isTemplate = false;

    if (!response.ok) {
      console.warn(`Instance ${jobId} not found, falling back to active profile cover letter template.`);
      response = await fetch(`/api/profile/active/cover-letter-template?t=${t}`);
      isTemplate = true;
    }
    
    if (isTemplate && jobId && jobId !== 'null') {
      document.body.innerHTML = `
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; font-family: 'Inter', sans-serif; background: #fff;">
            <button id="btn-generate-cover" style="
                background: #0052ff;
                color: white;
                border: none;
                padding: 14px 28px;
                border-radius: 12px;
                font-weight: 600;
                cursor: pointer;
                font-size: 15px;
            ">Générer la lettre</button>
        </div>
      `;
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
      return;
    }

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const data = await response.json();
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
  } catch (error) {
    console.error("Unable to load cover letter data:", error);
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
