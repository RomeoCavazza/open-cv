async function loadCoverLetter() {
  try {
    const response = await fetch("data.json");
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

const downloadButton = document.getElementById("download-pdf");
if (downloadButton) {
  downloadButton.addEventListener("click", () => window.print());
}

window.addEventListener('DOMContentLoaded', () => {
  if (window.self !== window.top) {
    document.body.classList.add("is-framed");
  }
  loadCoverLetter();
});
