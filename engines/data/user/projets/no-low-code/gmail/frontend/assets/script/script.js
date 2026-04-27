// === Configuration ===
  const { webhookUrl, jsonUrl } = window.APP_CONFIG;

// === Stockage local ===
const STORAGE = {
  PIN_KEY: "pinnedEmailIds",
  HIDE_KEY: "hiddenEmailIds",
  TRASH_KEY: "trashedEmailIds",
  ORDER_KEY: "emailsCustomOrder",
  
  get: (key) => JSON.parse(localStorage.getItem(key) || "[]"),
  set: (key, value) => localStorage.setItem(key, JSON.stringify(value))
};

// === État de l'application ===
const state = {
  emails: [],
  summary: null,
  pinned: new Set(STORAGE.get(STORAGE.PIN_KEY)),
  hidden: new Set(STORAGE.get(STORAGE.HIDE_KEY)),
  trashed: new Set(STORAGE.get(STORAGE.TRASH_KEY)),
  customOrder: JSON.parse(localStorage.getItem(STORAGE.ORDER_KEY) || "{}"),
  draggedRow: null
};

// === Éléments DOM ===
const DOM = {
  // Résumé
  sumDay: document.getElementById("sumDay"),
  sumTotal: document.getElementById("sumTotal"),
  urgencyBadge: document.getElementById("urgencyBadge"),
  urgencyText: document.getElementById("urgencyText"),
  sumPriority: document.getElementById("sumPriority"),
  prioritySection: document.getElementById("prioritySection"),
  sumTLDR: document.getElementById("sumTLDR"),
  sumTopics: document.getElementById("sumTopics"),
  topicsSection: document.getElementById("topicsSection"),

  // Liste et contrôles
  searchInput: document.getElementById("searchInput"),
  mailsTbody: document.getElementById("mailsTbody"),
  emptyState: document.getElementById("emptyState"),
  emptyTitle: document.getElementById("emptyTitle"),
  emptyDescription: document.getElementById("emptyDescription"),

  // Nouveaux filtres
  senderBtn: document.getElementById("senderFilterBtn"),
  senderDropdown: document.getElementById("senderDropdown"),
  senderList: document.getElementById("senderList"),
  senderSearchInput: document.getElementById("senderSearchInput"),
  senderLabel: document.getElementById("senderFilterLabel"),
  
  sortBtn: document.getElementById("sortBtn"),
  sortDropdown: document.getElementById("sortDropdown"),
  sortLabel: document.getElementById("sortLabel"),
  
  toggleShowDone: document.getElementById("toggleShowDone"),
  toggleShowLabel: document.getElementById("toggleShowLabel"),
  togglePinned: document.getElementById("togglePinned"),
  togglePinnedLabel: document.getElementById("togglePinnedLabel"),
  countPinned: document.getElementById("countPinned"),
  
  toggleTrash: document.getElementById("toggleTrash"),
  toggleTrashLabel: document.getElementById("toggleTrashLabel"),
  countTrash: document.getElementById("countTrash"),

  // Autres
  refreshBtn: document.getElementById("refreshBtn"),
  lastUpdated: document.getElementById("lastUpdated"),
  toast: document.getElementById("toast"),
  
  // Modal
  emailModal: document.getElementById("emailModal"),
  modalClose: document.getElementById("modalClose"),
  modalPrev: document.getElementById("modalPrev"),
  modalNext: document.getElementById("modalNext"),
  modalFrom: document.getElementById("modalFrom"),
  modalDate: document.getElementById("modalDate"),
  modalSubject: document.getElementById("modalSubject"),
  modalId: document.getElementById("modalId"),
  modalPin: document.getElementById("modalPin"),
  modalHide: document.getElementById("modalHide"),
  modalTrash: document.getElementById("modalTrash"),
  modalOpenGmail: document.getElementById("modalOpenGmail")
};

// === Utilitaires ===
const utils = {
  // Afficher notification toast discrète
  showToast: (msg, isSuccess = true) => {
    DOM.toast.textContent = msg;
    DOM.toast.classList.remove("hidden", "ok", "err");
    DOM.toast.classList.add(isSuccess ? "ok" : "err");
    setTimeout(() => DOM.toast.classList.add("hidden"), 2500);
  },
  
  // Formater une date ISO en français
  formatDate: (iso) => {
    if (!iso) return "—";
    try {
      return new Date(iso).toLocaleString("fr-FR", {
        year: "numeric", month: "short", day: "2-digit",
        hour: "2-digit", minute: "2-digit"
      }).replace(",", "");
    } catch {
      return iso;
    }
  },
  
  // Échapper HTML
  escapeHtml: (str = "") => {
    const map = { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#039;' };
    return str.replace(/[&<>"']/g, c => map[c]);
  },
  
  // Extraire l'adresse email
  extractEmail: (email) => {
    // Si from est une string simple, la retourner directement
    if (typeof email?.from === 'string') return email.from;
    // Sinon, structure complexe (ancienne logique)
    return email?.from?.value?.[0]?.address || "";
  },
  
  // Obtenir le nom d'affichage ou l'email
  getDisplayName: (email) => {
    // Si from est une string simple, la retourner directement
    if (typeof email?.from === 'string') return email.from;
    // Sinon, fallback sur from_display ou extractEmail
    return email.from_display || utils.extractEmail(email);
  },
  
  // Dédupliquer un tableau
  dedupe: (arr) => Array.from(new Set(arr))
};

// === Filtrage et tri ===
const filters = {
  // État actuel
  currentSender: '',
  currentSort: 'custom',
  senderSearch: '',
  
  // Récupérer les filtres actuels
  getCurrent: () => ({
    query: (DOM.searchInput.value || "").trim().toLowerCase(),
    sender: filters.currentSender,
    pinnedOnly: DOM.togglePinned?.getAttribute('aria-pressed') === 'true',
    showDone: DOM.toggleShowDone?.getAttribute('aria-pressed') === 'true',
    trashOnly: DOM.toggleTrash?.getAttribute('aria-pressed') === 'true'
  }),
  
  // Filtrer les emails
  apply: (emails) => {
    const { query, sender, pinnedOnly, showDone, trashOnly } = filters.getCurrent();
    
    return emails.filter(email => {
      const isDone = state.hidden.has(email.id);
      const isPinned = state.pinned.has(email.id);
      const isTrashed = state.trashed.has(email.id);
      
      // Toggle Corbeille : Afficher uniquement la corbeille ou tout sauf la corbeille
      if (trashOnly) {
        if (!isTrashed) return false; // Ne montrer que les emails en corbeille
      } else {
        if (isTrashed) return false; // Exclure les emails en corbeille de la vue normale
      }
      
      // Toggle 1 : Épinglés uniquement
      if (pinnedOnly && !isPinned) return false;
      
      // Toggle 2 : Afficher/masquer les faits
      if (!showDone && isDone) return false;
      
      // Filtre par expéditeur
      if (sender && utils.getDisplayName(email).toLowerCase() !== sender.toLowerCase()) {
        return false;
      }
      
      // Recherche textuelle
      if (query) {
        const searchText = [
          email.subject || "",
          utils.getDisplayName(email),
          utils.extractEmail(email)
      ].join(" ").toLowerCase();
        
        return searchText.includes(query);
      }
      
      return true;
    });
  },
  
  // Trier les emails (ÉPINGLÉS TOUJOURS EN HAUT)
  sort: (emails) => {
    const mode = filters.currentSort;
    
    // Si tri personnalisé (custom order)
    if (mode === "custom" && Object.keys(state.customOrder).length > 0) {
      return [...emails].sort((a, b) => {
        // Épinglés en premier
        const aPinned = state.pinned.has(a.id) ? 0 : 1;
        const bPinned = state.pinned.has(b.id) ? 0 : 1;
        if (aPinned !== bPinned) return aPinned - bPinned;
        
        // Puis ordre personnalisé
        const posA = state.customOrder[a.id] ?? 9999;
        const posB = state.customOrder[b.id] ?? 9999;
        return posA - posB;
      });
    }
    
    // Fonctions de comparaison
    const comparators = {
      dateDesc: (a, b) => (b.date || "").localeCompare(a.date || ""),
      dateAsc: (a, b) => (a.date || "").localeCompare(b.date || ""),
      senderAsc: (a, b) => utils.getDisplayName(a).localeCompare(utils.getDisplayName(b)),
      senderDesc: (a, b) => utils.getDisplayName(b).localeCompare(utils.getDisplayName(a))
    };
    
    // Tri avec ÉPINGLÉS TOUJOURS EN PREMIER
    return [...emails].sort((a, b) => {
      // 1. Épinglés en premier (priorité absolue)
      const aPinned = state.pinned.has(a.id) ? 0 : 1;
      const bPinned = state.pinned.has(b.id) ? 0 : 1;
      if (aPinned !== bPinned) return aPinned - bPinned;
      
      // 2. Puis tri selon le mode sélectionné
      const comparator = comparators[mode] || (() => 0);
      return comparator(a, b);
    });
  }
};

// === Composants Réutilisables ===
const components = {
  // Grab handle minimaliste
  grab: (email) => {
    const td = document.createElement("td");
    td.className = "td-grab";
    
    const grabHandle = document.createElement("span");
    grabHandle.className = "grab-handle";
    grabHandle.innerHTML = '<i data-lucide="grip-vertical" style="width:16px;height:16px;stroke-width:1.5"></i>';
    grabHandle.title = "Réorganiser";
    
    td.appendChild(grabHandle);
    return td;
  },
  
  // Pin (colonne à droite)
  pin: (email) => {
    const td = document.createElement("td");
    td.className = "td-pinned";
    const isPinned = state.pinned.has(email.id);
    const isTrashed = state.trashed.has(email.id);
    
    const pinBtn = document.createElement("button");
    pinBtn.className = `pin-btn ${isPinned ? 'pinned' : ''}`;
    pinBtn.innerHTML = isPinned 
      ? '<i data-lucide="pin" class="icon-pin"></i>' 
      : '<i data-lucide="pin" class="icon-pin"></i>';
    pinBtn.title = isPinned ? 'Désépingler' : 'Épingler';
    pinBtn.ariaLabel = isPinned ? 'Désépingler' : 'Épingler';
    pinBtn.disabled = isTrashed; // Désactiver si en corbeille
    pinBtn.onclick = (e) => {
      e.stopPropagation();
      if (!isTrashed) emailActions.pin(email.id);
    };
    
    td.appendChild(pinBtn);
    return td;
  },
  
  // Expéditeur (lien direct vers Gmail)
  sender: (email) => {
    const td = document.createElement("td");
    td.className = "td-sender";
    
    const senderName = utils.getDisplayName(email);
    const emailAddress = utils.extractEmail(email);
    
    const link = document.createElement("a");
    link.href = `https://mail.google.com/mail/u/0/#search/from:${encodeURIComponent(emailAddress)}`;
    link.target = "_blank";
    link.className = "sender-link";
    link.textContent = senderName;
    link.title = `Voir tous les emails de ${senderName}`;
    
    // Empêcher l'ouverture du modal
    link.onclick = (e) => {
      e.stopPropagation();
    };
    
    td.appendChild(link);
    return td;
  },
  
  // Objet (normal)
  subject: (email) => {
    const td = document.createElement("td");
    td.className = "td-subject";
    td.textContent = email.subject || "—";
    return td;
  },
  
  // Heure uniquement (pas de date)
  time: (email) => {
    const td = document.createElement("td");
    td.className = "td-time";
    const date = new Date(email.date);
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    td.textContent = `${hours}:${minutes}`;
    return td;
  },
  
  // Status ○/✓ (hide/show) - RÉVERSIBLE
  status: (email) => {
    const td = document.createElement("td");
    td.className = "td-status";
    const isDone = state.hidden.has(email.id);
    const isTrashed = state.trashed.has(email.id);
    
    const indicator = document.createElement("button");
    indicator.className = `status-indicator ${isDone ? 'checked' : 'unchecked'}`;
    indicator.innerHTML = isDone 
      ? '<i data-lucide="eye-off" class="icon-eye"></i>' 
      : '<i data-lucide="eye" class="icon-eye"></i>';
    indicator.title = isDone ? 'Réactiver' : 'Archiver';
    indicator.ariaLabel = isDone ? 'Réactiver' : 'Archiver';
    indicator.disabled = isTrashed; // Désactiver si en corbeille
    indicator.onclick = (e) => {
      e.stopPropagation();
      if (!isTrashed) emailActions.hide(email.id);
    };
    
    td.appendChild(indicator);
    return td;
  },
  
  // Corbeille (nouvelle colonne)
  trash: (email) => {
    const td = document.createElement("td");
    td.className = "td-trash";
    const isTrashed = state.trashed.has(email.id);
    
    const trashBtn = document.createElement("button");
    trashBtn.className = `trash-btn ${isTrashed ? 'trashed' : ''}`;
    trashBtn.innerHTML = isTrashed 
      ? '<i data-lucide="archive-restore" class="icon-trash"></i>' 
      : '<i data-lucide="trash-2" class="icon-trash"></i>';
    trashBtn.title = isTrashed ? 'Restaurer' : 'Mettre à la corbeille';
    trashBtn.ariaLabel = isTrashed ? 'Restaurer' : 'Supprimer';
    trashBtn.onclick = (e) => {
      e.stopPropagation();
      emailActions.trash(email.id);
    };
    
    td.appendChild(trashBtn);
    return td;
  }
};

// === Rendu ===
const render = {
  // Afficher le résumé AI enrichi
  summary: (data) => {
    data = data || {};  // Forcer à {} si null/undefined
    DOM.sumDay.textContent = data.day ?? "—";
    DOM.sumTotal.textContent = (data.total ?? 0).toString();
    
    // Badge urgence (inline dans le header AI)
    if (data.urgency_level) {
      DOM.urgencyBadge.classList.remove('hidden');
      DOM.urgencyBadge.setAttribute('data-level', data.urgency_level);
      const urgencyMap = {
        'faible': 'Faible',
        'moyenne': 'Moyenne',
        'élevée': 'Élevée'
      };
      DOM.urgencyText.textContent = urgencyMap[data.urgency_level] || 'Faible';
    } else {
      DOM.urgencyBadge.classList.add('hidden');
    }
    
    // Emails prioritaires
    if (data.priority_emails && data.priority_emails.length > 0) {
      DOM.prioritySection.classList.remove('hidden');
      DOM.sumPriority.innerHTML = data.priority_emails.map(email => 
        `<div class="priority-item">
          <i data-lucide="chevron-right" class="priority-icon"></i>
          <span class="priority-text">${utils.escapeHtml(email)}</span>
        </div>`
      ).join('');
      if (window.lucide) lucide.createIcons();
    } else {
      DOM.prioritySection.classList.add('hidden');
    }
    
    // TL;DR amélioré
    DOM.sumTLDR.textContent = data.tl_dr ?? "—";
    
    // Thèmes clés (tags inline avec section)
    if (data.key_topics && data.key_topics.length > 0) {
      DOM.topicsSection.classList.remove('hidden');
      DOM.sumTopics.innerHTML = data.key_topics.slice(0, 5).map(topic => 
        `<span class="topic-tag-inline">${utils.escapeHtml(topic)}</span>`
      ).join('');
      if (window.lucide) lucide.createIcons();
    } else {
      DOM.topicsSection.classList.add('hidden');
    }
  },
  
  // États vides avec messages contextuels
  emptyState: () => {
    if (!DOM.emptyState || !DOM.emptyTitle || !DOM.emptyDescription) return;
    
    const states = {
      trash: {
        emoji: '🗑️',
        title: 'Corbeille vide',
        description: 'Aucun email dans la corbeille. Tout est propre ! 🎉'
      },
      pinned: {
        emoji: '📌',
        title: 'Aucun email épinglé',
        description: 'Épinglez vos emails importants pour les retrouver facilement'
      },
      search: {
        emoji: '🔍',
        title: 'Aucun résultat',
        description: 'Essayez avec d\'autres mots-clés ou vérifiez l\'orthographe'
      },
      sender: {
        emoji: '👤',
        title: 'Aucun email de cet expéditeur',
        description: 'Sélectionnez un autre expéditeur ou réinitialisez le filtre'
      },
      noEmails: {
        emoji: '📭',
        title: 'Boîte de réception vide',
        description: 'Profitez de ce moment de calme ! ☕'
      },
      default: {
        emoji: '🔎',
        title: 'Aucun e-mail trouvé',
        description: 'Modifiez vos filtres pour voir plus de résultats'
      }
    };
    
    let currentState = 'default';
    const current = filters.getCurrent();
    
    // Détection du contexte
    if (state.emails.length === 0) {
      currentState = 'noEmails';
    } else if (current.trashOnly) {
      currentState = 'trash';
    } else if (current.pinnedOnly) {
      currentState = 'pinned';
    } else if (current.query) {
      currentState = 'search';
    } else if (filters.currentSender) {
      currentState = 'sender';
    }
    
    const msg = states[currentState];
    const emoji = DOM.emptyState.querySelector('.empty-emoji');
    if (emoji) emoji.textContent = msg.emoji;
    DOM.emptyTitle.textContent = msg.title;
    DOM.emptyDescription.textContent = msg.description;
    DOM.emptyState.classList.remove('hidden');
  },
  
  // Créer une ligne de tableau (SIMPLIFIÉ avec composants)
  createRow: (email) => {
    const tr = document.createElement("tr");
    
    // Classes CSS
    const classes = [];
    const isDone = state.hidden.has(email.id);
    const isPinned = state.pinned.has(email.id);
    const isTrashed = state.trashed.has(email.id);
    
    // RÈGLE : Épinglés ne sont jamais grisés (même si faits)
    if (isDone && !isPinned && !isTrashed) {
      classes.push("done");
    }
    
    // Emails en corbeille ont un style spécial
    if (isTrashed) {
      classes.push("trashed");
    }
    
    tr.className = classes.join(" ");
    
    // Drag & drop
    tr.draggable = true;
    tr.dataset.emailId = email.id;
    tr.addEventListener('dragstart', dragDrop.handleDragStart);
    tr.addEventListener('dragover', dragDrop.handleDragOver);
    tr.addEventListener('drop', dragDrop.handleDrop);
    tr.addEventListener('dragend', dragDrop.handleDragEnd);
    tr.addEventListener('dragenter', dragDrop.handleDragEnter);
    tr.addEventListener('dragleave', dragDrop.handleDragLeave);
    
    // Clic sur la ligne pour ouvrir le modal
    tr.addEventListener('click', (e) => {
      // Ne pas ouvrir si on clique sur un bouton d'action
      if (e.target.closest('button')) return;
      modal.open(email);
    });
    
    // Assembler : Grab | Expéditeur | Objet | Heure | Pin | Status | Trash
    // Ordre cohérent avec les filtres : Pin -> Show/Hide -> Trash
    tr.append(
      components.grab(email),
      components.sender(email),
      components.subject(email),
      components.time(email),
      components.pin(email),
      components.status(email),
      components.trash(email)
    );
    
    return tr;
  },
  
  // DÉCOUPÉ : Obtenir emails filtrés et triés
  getSorted: () => {
    const filtered = filters.apply(state.emails);
    return filters.sort(filtered);
  },
  
  // DÉCOUPÉ : Séparer actifs et done (ÉPINGLÉS RESTENT EN HAUT)
  splitByStatus: (emails) => {
    const active = [];
    const done = [];
    
    emails.forEach(e => {
      const isDone = state.hidden.has(e.id);
      const isPinned = state.pinned.has(e.id);
      const isTrashed = state.trashed.has(e.id);
      
      // Si en corbeille, traiter différemment (grisé mais pas séparé)
      if (isTrashed) {
        active.push(e); // Les emails en corbeille restent visibles mais stylés différemment
        return;
      }
      
      // RÈGLE : Épinglés restent en haut, même si faits
      if (isPinned) {
        active.push(e);  // Épinglés toujours dans "active" (en haut)
      } else if (isDone) {
        done.push(e);    // Non-épinglés faits vont en bas
      } else {
        active.push(e);  // Non-épinglés actifs
      }
    });
    
    return { active, done };
  },
  
  // DÉCOUPÉ : Remplir le tableau
  fillTable: (active, done) => {
    DOM.mailsTbody.innerHTML = "";
    [...active, ...done].forEach(email => {
      DOM.mailsTbody.appendChild(render.createRow(email));
    });
  },
  
  // DÉCOUPÉ : Mettre à jour compteurs avec logique stricte
  updateCounters: (active, done) => {
    const setCount = (id, value) => {
      const el = document.getElementById(id);
      if (el) {
        el.textContent = value;
        // Masquer le badge si 0
        el.style.display = value > 0 ? 'inline-flex' : 'none';
      }
    };
    
    // Compteur épinglés : seulement ceux NON en corbeille
    const pinnedCount = state.emails.filter(e => 
      state.pinned.has(e.id) && !state.trashed.has(e.id)
    ).length;
    setCount('countPinned', pinnedCount);
    
    // Compteur corbeille : tous les emails en corbeille
    const trashedCount = state.emails.filter(e => 
      state.trashed.has(e.id)
    ).length;
    setCount('countTrash', trashedCount);
  },
  
  // ORCHESTRATEUR : Afficher la table complète
  table: () => {
    const sorted = render.getSorted();
    const { active, done } = render.splitByStatus(sorted);
    
    // Gestion de l'affichage : tableau OU état vide
    if (sorted.length === 0) {
      // Vider le tableau et cacher le wrapper
      DOM.mailsTbody.innerHTML = '';
      const tableWrap = document.querySelector('.table-wrap');
      if (tableWrap) tableWrap.style.display = 'none';
      render.emptyState();
    } else {
      // Afficher le tableau, cacher l'état vide
      const tableWrap = document.querySelector('.table-wrap');
      if (tableWrap) tableWrap.style.display = 'block';
      DOM.emptyState.classList.add('hidden');
      render.fillTable(active, done);
      render.updateCounters(active, done);
    }
    
    // Date de mise à jour
    DOM.lastUpdated.textContent = `Mis à jour : ${new Date().toLocaleString("fr-FR")}`;
    
    // Initialiser toutes les icônes Lucide après le rendu
    if (window.lucide) {
      lucide.createIcons();
    }
  },
  
  // Peupler la liste des expéditeurs
  senderOptions: (emails) => {
    // Compter le nombre d'emails par expéditeur
    const senderCounts = {};
    emails.forEach(e => {
      const name = utils.getDisplayName(e).trim();
      if (name) {
        senderCounts[name] = (senderCounts[name] || 0) + 1;
      }
    });
    
    // Trier par nombre d'emails (décroissant)
    const sortedSenders = Object.entries(senderCounts)
      .sort((a, b) => b[1] - a[1])
      .map(([name, count]) => ({ name, count }));
    
    DOM.senderList.innerHTML = `
      <button class="dropdown-item ${!filters.currentSender ? 'active' : ''}" data-sender="">
        <i data-lucide="users" class="dropdown-icon"></i>
        <span>Tous les expéditeurs</span>
        <span class="badge-count">${emails.length}</span>
      </button>
      ${sortedSenders.map(({ name, count }) => `
        <button class="dropdown-item ${filters.currentSender === name ? 'active' : ''}" data-sender="${utils.escapeHtml(name)}">
          <i data-lucide="user" class="dropdown-icon"></i>
          <span>${utils.escapeHtml(name)}</span>
          ${count >= 2 ? `<span class="badge-count">${count}</span>` : ''}
        </button>
      `).join('')}
    `;

    // Initialiser les icônes Lucide
    if (window.lucide) {
      lucide.createIcons();
    }

    // Event listeners pour les items
    DOM.senderList.querySelectorAll('.dropdown-item').forEach(item => {
      item.addEventListener('click', () => {
        const sender = item.dataset.sender;
        filters.currentSender = sender;
        
        // Mettre à jour le label (court)
        if (!sender) {
          DOM.senderLabel.textContent = 'Tous';
        } else {
          DOM.senderLabel.textContent = sender.length > 20 
            ? sender.substring(0, 20) + '...' 
            : sender;
        }

        // Mettre à jour les items actifs
        DOM.senderList.querySelectorAll('.dropdown-item').forEach(i => {
          i.classList.toggle('active', i.dataset.sender === sender);
        });

        // Fermer le dropdown
        DOM.senderDropdown.classList.add('hidden');
        DOM.senderBtn.setAttribute('aria-expanded', 'false');
        
        // Re-render
        render.table();
      });
    });
  }
};

// === Gestion des dropdowns ===
const dropdowns = {
  toggleSender: (e) => {
    e.stopPropagation();
    const isOpen = !DOM.senderDropdown.classList.contains('hidden');
    dropdowns.closeAll();
    
    if (!isOpen) {
      DOM.senderDropdown.classList.remove('hidden');
      DOM.senderBtn.setAttribute('aria-expanded', 'true');
      setTimeout(() => DOM.senderSearchInput?.focus(), 100);
    }
  },

  toggleSort: (e) => {
    e.stopPropagation();
    const isOpen = !DOM.sortDropdown.classList.contains('hidden');
    dropdowns.closeAll();
    
    if (!isOpen) {
      DOM.sortDropdown.classList.remove('hidden');
      DOM.sortBtn.setAttribute('aria-expanded', 'true');
    }
  },

  closeAll: () => {
    DOM.senderDropdown?.classList.add('hidden');
    DOM.sortDropdown?.classList.add('hidden');
    DOM.senderBtn?.setAttribute('aria-expanded', 'false');
    DOM.sortBtn?.setAttribute('aria-expanded', 'false');
    
    // Reset recherche expéditeur
    if (DOM.senderSearchInput) {
      DOM.senderSearchInput.value = '';
      dropdowns.filterSenderList();
    }
  },

  filterSenderList: () => {
    const query = (DOM.senderSearchInput?.value || '').toLowerCase();
    DOM.senderList?.querySelectorAll('.dropdown-item').forEach(item => {
      const text = item.textContent.toLowerCase();
      item.style.display = text.includes(query) ? 'flex' : 'none';
    });
  }
};

// === Chargement des données ===
const data = {
  // Récupérer les données JSON
  fetch: async () => {
    const url = `${jsonUrl}?t=${Date.now()}`;
    const res = await fetch(url, { cache: "no-store" });
    if (!res.ok) throw new Error(`Erreur ${res.status}`);
    return res.json();
  },
  
  // Charger et afficher
  load: async () => {
    try {
      const json = await data.fetch();
      state.emails = json?.emails || [];
      // Le résumé est directement dans json (pas dans json.summary)
      state.summary = json || null;
      
      render.summary(state.summary);
      render.senderOptions(state.emails);
      render.table();
    } catch (err) {
      utils.showToast(`Erreur de chargement: ${err.message}`, false);
      console.error(err);
    }
  },

  // Actualiser via webhook
  refresh: async () => {
    try {
      DOM.refreshBtn.disabled = true;
      const res = await fetch(webhookUrl, { method: "POST" });
      if (!res.ok) throw new Error(`Webhook ${res.status}`);
      
      utils.showToast("Actualisation lancée ✅");
      await data.load();
    } catch (err) {
      utils.showToast("Échec de l'actualisation ❌", false);
      console.error(err);
    } finally {
      DOM.refreshBtn.disabled = false;
    }
  }
};

// === Actions Universelles sur Emails ===
const emailActions = {
  // Fonction universelle toggle (pin ou hide ou trash)
  toggle: (type, emailIds) => {
    const ids = Array.isArray(emailIds) ? emailIds : [emailIds];
    const stateSet = type === 'pin' ? state.pinned : (type === 'hide' ? state.hidden : state.trashed);
    const storageKey = type === 'pin' ? STORAGE.PIN_KEY : (type === 'hide' ? STORAGE.HIDE_KEY : STORAGE.TRASH_KEY);
    
    // Déterminer si tous sont déjà actifs
    const allActive = ids.every(id => stateSet.has(id));
    
    // Toggle
    ids.forEach(id => {
      if (allActive) {
        stateSet.delete(id);
      } else {
        stateSet.add(id);
      }
    });
    
    // Sauvegarder
    STORAGE.set(storageKey, Array.from(stateSet));
    
    // Toast harmonieux
    const messages = {
      pin: allActive ? 'Désépinglé' : 'Épinglé',
      hide: allActive ? 'Réactivé' : 'Archivé',
      trash: allActive ? 'Restauré' : 'Mis à la corbeille'
    };
    const prefix = ids.length > 1 ? `${ids.length} emails` : 'Email';
    utils.showToast(`${prefix} ${messages[type].toLowerCase()}`, true);
    
    // Re-render
    render.table();
  },
  
  // Raccourcis
  pin: (ids) => emailActions.toggle('pin', ids),
  hide: (ids) => emailActions.toggle('hide', ids),
  trash: (ids) => emailActions.toggle('trash', ids)
};

// === Modal Email avec Navigation ===
const modal = {
  currentEmail: null,
  currentIndex: -1,
  visibleEmails: [],
  
  open: (email) => {
    modal.currentEmail = email;
    
    // Trouver l'index dans la liste visible
    modal.visibleEmails = Array.from(DOM.mailsTbody.querySelectorAll('tr')).map(tr => {
      const id = tr.dataset.emailId;
      return state.emails.find(e => e.id === id);
    }).filter(Boolean);
    
    modal.currentIndex = modal.visibleEmails.findIndex(e => e.id === email.id);
    
    modal.updateContent();
    
    // Afficher le modal
    DOM.emailModal.classList.remove('hidden');
    document.body.style.overflow = 'hidden';
    
    // Initialiser les icônes Lucide
    if (window.lucide) lucide.createIcons();
  },
  
  updateContent: () => {
    const email = modal.currentEmail;
    if (!email) return;
    
    // Remplir les données essentielles
    DOM.modalSubject.textContent = email.subject || "Sans objet";
    DOM.modalFrom.textContent = utils.getDisplayName(email);
    DOM.modalDate.textContent = utils.formatDate(email.date);
    
    // Lien Gmail
    const searchQuery = encodeURIComponent(`subject:"${email.subject || ''}"`);
    DOM.modalOpenGmail.href = `https://mail.google.com/mail/u/0/#search/${searchQuery}`;
    
    // États des boutons d'action
    const isPinned = state.pinned.has(email.id);
    const isHidden = state.hidden.has(email.id);
    const isTrashed = state.trashed.has(email.id);
    
    DOM.modalPin.classList.toggle('active', isPinned);
    DOM.modalHide.classList.toggle('active', isHidden);
    DOM.modalTrash.classList.toggle('active', isTrashed);
    
    // Icônes dynamiques
    DOM.modalPin.querySelector('.action-icon').setAttribute('data-lucide', 'pin');
    DOM.modalHide.querySelector('.action-icon').setAttribute('data-lucide', isHidden ? 'eye-off' : 'eye');
    DOM.modalTrash.querySelector('.action-icon').setAttribute('data-lucide', isTrashed ? 'archive-restore' : 'trash-2');
    
    // Navigation précédent/suivant
    DOM.modalPrev.disabled = modal.currentIndex <= 0;
    DOM.modalNext.disabled = modal.currentIndex >= modal.visibleEmails.length - 1;
    
    // Rafraîchir les icônes
    if (window.lucide) lucide.createIcons();
  },
  
  navigate: (direction) => {
    const newIndex = modal.currentIndex + direction;
    if (newIndex >= 0 && newIndex < modal.visibleEmails.length) {
      modal.currentIndex = newIndex;
      modal.currentEmail = modal.visibleEmails[newIndex];
      modal.updateContent();
    }
  },
  
  close: () => {
    DOM.emailModal.classList.add('hidden');
    document.body.style.overflow = '';
    modal.currentEmail = null;
    modal.currentIndex = -1;
    modal.visibleEmails = [];
  }
};

// === Drag & Drop simple pour réorganiser ===
const dragDrop = {
  handleDragStart: function(e) {
    state.draggedRow = this;
    state.draggedEmail = this.dataset.emailId;
    
    this.classList.add('dragging');
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/html', this.innerHTML);
  },
  
  handleDragOver: function(e) {
    if (e.preventDefault) e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    return false;
  },
  
  handleDragEnter: function(e) {
    if (this === state.draggedRow) return;
    this.classList.add('drag-over');
  },
  
  handleDragLeave: function(e) {
    this.classList.remove('drag-over');
  },
  
  handleDrop: function(e) {
    if (e.stopPropagation) e.stopPropagation();
    
    if (state.draggedRow !== this) {
      // Réorganiser
      const tbody = DOM.mailsTbody;
      const allRows = Array.from(tbody.querySelectorAll('tr'));
      const draggedIndex = allRows.indexOf(state.draggedRow);
      const targetIndex = allRows.indexOf(this);
      
      // Animation smooth de réorganisation
      if (draggedIndex < targetIndex) {
        tbody.insertBefore(state.draggedRow, this.nextSibling);
      } else {
        tbody.insertBefore(state.draggedRow, this);
      }
      
      // Mettre à jour l'ordre personnalisé
      dragDrop.updateCustomOrder();
      
      // Passer en mode tri personnalisé
      filters.currentSort = 'custom';
      DOM.sortLabel.textContent = 'Personnalisé';
    }
    
    return false;
  },
  
  handleDragEnd: function(e) {
    this.classList.remove('dragging');
    
    // Retirer tous les drag-over
    document.querySelectorAll('.drag-over').forEach(el => {
      el.classList.remove('drag-over');
    });
  },
  
  updateCustomOrder: () => {
    const rows = Array.from(DOM.mailsTbody.querySelectorAll('tr'));
    const newOrder = {};
    
    rows.forEach((row, index) => {
      const emailId = row.dataset.emailId;
      if (emailId) {
        newOrder[emailId] = index;
      }
    });
    
    state.customOrder = newOrder;
    localStorage.setItem(STORAGE.ORDER_KEY, JSON.stringify(newOrder));
    
    utils.showToast("Ordre personnalisé enregistré ✅", true);
  }
};

// === Event Listeners ===
DOM.refreshBtn.addEventListener("click", data.refresh);
DOM.searchInput.addEventListener("input", render.table);

// Dropdowns
DOM.senderBtn.addEventListener("click", dropdowns.toggleSender);
DOM.sortBtn.addEventListener("click", dropdowns.toggleSort);
DOM.senderSearchInput?.addEventListener("input", dropdowns.filterSenderList);
document.addEventListener("click", (e) => {
  if (!e.target.closest('.filter-item')) {
    dropdowns.closeAll();
  }
});

// Items de tri
DOM.sortDropdown.querySelectorAll('.dropdown-item').forEach(item => {
  item.addEventListener('click', () => {
    const sortMode = item.dataset.sort;
    filters.currentSort = sortMode;
    
    // Mettre à jour le label (utiliser data-sort-label si disponible)
    DOM.sortLabel.textContent = item.dataset.sortLabel || item.textContent.trim();
    
    // Mettre à jour les items actifs
    DOM.sortDropdown.querySelectorAll('.dropdown-item').forEach(i => {
      i.classList.toggle('active', i.dataset.sort === sortMode);
    });
    
    // Fermer le dropdown
    dropdowns.closeAll();
    
    // Re-render
    render.table();
  });
});

// Toggle Affichage - LOGIQUE ARIA CORRECTE
DOM.toggleShowDone.addEventListener('click', () => {
  const showDone = DOM.toggleShowDone.getAttribute('aria-pressed') === 'true';
  DOM.toggleShowDone.setAttribute('aria-pressed', !showDone);
  // aria-pressed="true" (ON/bleu) = état par défaut = tout affiché
  // aria-pressed="false" (OFF/blanc) = filtré = archivés masqués
  DOM.toggleShowLabel.textContent = !showDone ? 'Complet' : 'Actifs';
  render.table();
});

DOM.togglePinned.addEventListener('click', () => {
  const pinnedOnly = DOM.togglePinned.getAttribute('aria-pressed') === 'true';
  DOM.togglePinned.setAttribute('aria-pressed', !pinnedOnly);
  // aria-pressed="false" (OFF/blanc) = état par défaut = tous
  // aria-pressed="true" (ON/bleu) = filtré = épinglés seulement
  DOM.togglePinnedLabel.textContent = !pinnedOnly ? 'Tous' : 'Épinglés';
  render.table();
});

DOM.toggleTrash.addEventListener('click', () => {
  const trashOnly = DOM.toggleTrash.getAttribute('aria-pressed') === 'true';
  DOM.toggleTrash.setAttribute('aria-pressed', !trashOnly);
  // aria-pressed="false" (OFF/blanc) = état par défaut = boîte
  // aria-pressed="true" (ON/bleu) = filtré = corbeille
  DOM.toggleTrashLabel.textContent = !trashOnly ? 'Boîte' : 'Corbeille';
  
  // Changer l'icône
  const icon = DOM.toggleTrash.querySelector('.filter-icon');
  if (icon) {
    icon.setAttribute('data-lucide', !trashOnly ? 'inbox' : 'trash-2');
    if (window.lucide) lucide.createIcons();
  }
  
  render.table();
});

// === 🪟 Event Listeners Modal ===
DOM.modalClose.addEventListener('click', modal.close);
DOM.emailModal.querySelector('.modal-overlay').addEventListener('click', modal.close);

// Navigation
DOM.modalPrev.addEventListener('click', () => modal.navigate(-1));
DOM.modalNext.addEventListener('click', () => modal.navigate(1));

// Actions email depuis le modal
DOM.modalPin.addEventListener('click', () => {
  emailActions.pin(modal.currentEmail.id);
  modal.updateContent();
});

DOM.modalHide.addEventListener('click', () => {
  emailActions.hide(modal.currentEmail.id);
  modal.updateContent();
});

DOM.modalTrash.addEventListener('click', () => {
  emailActions.trash(modal.currentEmail.id);
  modal.updateContent();
});

// Fermer avec Escape, naviguer avec flèches
document.addEventListener('keydown', (e) => {
  if (DOM.emailModal.classList.contains('hidden')) return;
  
  if (e.key === 'Escape') {
    modal.close();
  } else if (e.key === 'ArrowLeft') {
    e.preventDefault();
    modal.navigate(-1);
  } else if (e.key === 'ArrowRight') {
    e.preventDefault();
    modal.navigate(1);
  }
});

// === 🍬 UX Enhancements ===


// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
  // "/" pour focus sur recherche
  if (e.key === '/' && !['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName)) {
    e.preventDefault();
    DOM.searchInput.focus();
  }
  
  // "r" pour refresh
  if (e.key === 'r' && e.ctrlKey) {
    e.preventDefault();
    data.refresh();
  }
});


// Add ripple effect to buttons
document.querySelectorAll('.btn, .btn-action, .filter-chip').forEach(button => {
  button.addEventListener('click', function(e) {
    const ripple = document.createElement('span');
    const rect = this.getBoundingClientRect();
    const size = Math.max(rect.width, rect.height);
    const x = e.clientX - rect.left - size / 2;
    const y = e.clientY - rect.top - size / 2;
    
    ripple.style.cssText = `
      position: absolute;
      width: ${size}px;
      height: ${size}px;
      border-radius: 50%;
      background: rgba(255, 255, 255, 0.5);
      pointer-events: none;
      left: ${x}px;
      top: ${y}px;
      animation: ripple-effect 0.6s ease-out;
    `;
    
    this.appendChild(ripple);
    setTimeout(() => ripple.remove(), 600);
  });
});







// Add CSS for ripple animation
const style = document.createElement('style');
style.textContent = `
  @keyframes ripple-effect {
    to {
      transform: scale(4);
      opacity: 0;
    }
  }
`;
document.head.appendChild(style);


// === Initialisation ===
data.load();