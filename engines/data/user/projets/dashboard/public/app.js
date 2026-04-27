/**
 * Dashboard Logic for Ligue 1
 * Fetches data from local Mock Node Server
 */

document.addEventListener('DOMContentLoaded', () => {
    initDashboard();
});

async function initDashboard() {
    console.log("Initializing Dashboard...");
    try {
        // Parallel fetch for better performance
        // Note: Using relative paths assuming server.js serves this on /
        // Helper to check response
        const fetchProxy = async (url) => {
            const res = await fetch(url);
            if (!res.ok) {
                const text = await res.text();
                throw new Error(`Erreur ${res.status}: ${text}`);
            }
            return res.json();
        };

        const [compData, standingsData, matchesData] = await Promise.all([
            fetchProxy('/api/proxy?endpoint=/competitions/FL1'),
            fetchProxy('/api/proxy?endpoint=/competitions/FL1/standings'),
            fetchProxy('/api/proxy?endpoint=/competitions/FL1/matches')
        ]);

        console.log("Data fetched successfully");

        // ... (Rendering logic) ...

        // 1. Update Header Info
        updateHeader(compData);

        // 2. Render Main Standings Table
        const standings = standingsData.standings?.[0]?.table || [];
        renderStandings(standings);

        // 3. Render Upcoming Matches
        const matches = matchesData.matches || [];
        renderMatches(matches);

        // 4. Calculate KPI Cards
        calculateKPIs(standings);

        // 5. Render Top 5 Chart
        renderChart(standings.slice(0, 5));

    } catch (error) {
        console.error("Critical Error:", error);
        document.querySelector('.dashboard-grid').innerHTML =
            `<div style="padding:20px; color:var(--danger); text-align:center;">
                <h3>Oups ! Erreur de chargement 😕</h3>
                <p>${error.message}</p>
                <p style="font-size:0.8rem; color:var(--text-secondary); margin-top:10px;">
                    Vérifie que ta variable <code>API_KEY</code> est bien configurée sur Vercel et que tu as redéployé.
                </p>
            </div>`;
    }
}

/**
 * Updates header with Competition Name, Logo, Season
 */
function updateHeader(data) {
    if (!data) return;

    // Confirmed fields: name, emblem, currentSeason
    const nameEl = document.getElementById('league-name');
    const logoEl = document.getElementById('league-logo');
    const seasonEl = document.getElementById('season-dates');
    const matchdayEl = document.getElementById('current-matchday');

    if (nameEl) nameEl.textContent = data.name;
    if (logoEl) logoEl.src = data.emblem;

    if (data.currentSeason) {
        const start = data.currentSeason.startDate?.split('-')[0] || '';
        const end = data.currentSeason.endDate?.split('-')[0] || '';
        if (seasonEl) seasonEl.textContent = `Saison ${start}/${end}`;
        if (matchdayEl) matchdayEl.textContent = data.currentSeason.currentMatchday;
    }
}

/**
 * Renders the classification table
 */
function renderStandings(table) {
    const tbody = document.getElementById('standings-body');
    if (!tbody) return;

    tbody.innerHTML = '';

    table.forEach(row => {
        const tr = document.createElement('tr');

        // Confirmed fields: position, team.name/crest, points, playedGames, etc.
        const diffClass = row.goalDifference > 0 ? 'success' : (row.goalDifference < 0 ? 'danger' : '');
        const diffSign = row.goalDifference > 0 ? '+' : '';

        tr.innerHTML = `
            <td class="center" style="font-weight:bold;">${row.position}</td>
            <td class="left">
                <div class="team-cell">
                    <img src="${row.team.crest}" alt="${row.team.tla}" class="team-crest" onerror="this.src='https://via.placeholder.com/24'">
                    <span>${row.team.name}</span>
                </div>
            </td>
            <td class="center"><strong style="color:var(--accent-color);">${row.points}</strong></td>
            <td class="center mobile-hide">${row.playedGames}</td>
            <td class="center mobile-hide">${row.won}</td>
            <td class="center mobile-hide">${row.draw}</td>
            <td class="center mobile-hide">${row.lost}</td>
            <td class="center ${diffClass}" style="opacity:0.8;">${diffSign}${row.goalDifference}</td>

        `;
        tbody.appendChild(tr);
    });
}

/**
 * Renders upcoming matches in sidebar
 */
function renderMatches(matches) {
    const list = document.getElementById('matches-list');
    if (!list) return;

    list.innerHTML = '';

    // Filter SCHEDULED/TIMED and sort by date
    const upcoming = matches
        .filter(m => m.status === 'TIMED' || m.status === 'SCHEDULED' || m.status === 'IN_PLAY')
        .sort((a, b) => new Date(a.utcDate) - new Date(b.utcDate))
        .slice(0, 5); // Show max 5

    // Update matchday indicator in sidebar header if available
    if (upcoming.length > 0) {
        const nextMd = upcoming[0].matchday;
        const mdEl = document.getElementById('next-matchday-num');
        if (mdEl) mdEl.textContent = nextMd;
    } else {
        list.innerHTML = '<div style="padding:15px; text-align:center; color:#555;">Aucun match programmé</div>';
        return;
    }

    upcoming.forEach(m => {
        const date = new Date(m.utcDate);
        const day = date.toLocaleDateString('fr-FR', { day: 'numeric', month: 'short' });
        const time = date.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });

        const div = document.createElement('div');
        div.className = 'match-item';
        div.innerHTML = `
            <div class="match-info">
                <div class="match-meta">
                    <span class="match-status">${day} - ${time}</span>    
                </div>
                <div class="teams-vs">
                    <span>${m.homeTeam.shortName || m.homeTeam.name}</span>
                    <span style="color:var(--text-secondary); font-size:0.8em;">vs</span>
                    <span>${m.awayTeam.shortName || m.awayTeam.name}</span>
                </div>
            </div>
        `;
        list.appendChild(div);
    });
}

/**
 * Calculates and updates Top 4 KPIs
 */
function calculateKPIs(table) {
    // 1. Matches Played
    // Logic: Sum of all playedGames / 2
    const totalPlayed = table.reduce((acc, row) => acc + row.playedGames, 0) / 2;
    updateKpi('kpi-played', Math.floor(totalPlayed));

    // 2. Total Goals
    // Logic: Sum of goalsFor
    const totalGoals = table.reduce((acc, row) => acc + row.goalsFor, 0);
    updateKpi('kpi-goals', totalGoals);

    // 3. Avg Goals
    const avg = totalPlayed > 0 ? (totalGoals / totalPlayed).toFixed(2) : '-';
    updateKpi('kpi-avg-goals', avg);

    // 4. Current Leader
    if (table.length > 0) {
        // Position 1
        updateKpi('kpi-leader', table[0].team.name);
    }
}

function updateKpi(id, value) {
    const el = document.getElementById(id);
    if (el) el.textContent = value;
}

/**
 * Renders simple CSS Bar Chart
 */
function renderChart(topTeams) {
    const container = document.getElementById('chart-container');
    if (!container || topTeams.length === 0) return;

    container.innerHTML = '';

    // Find max points for scale
    const maxPts = Math.max(...topTeams.map(t => t.points));

    topTeams.forEach(t => {
        const percent = (t.points / maxPts) * 100;

        const row = document.createElement('div');
        row.className = 'chart-bar-row';
        row.innerHTML = `
            <div class="chart-label">${t.position}</div>
            <div style="width:20px; text-align:center; margin-right:8px;">
                 <img src="${t.team.crest}" style="width:18px; height:18px; vertical-align:middle;">
            </div>
            <div style="width: 40px; font-weight:600; font-size:0.75rem;">${t.team.tla}</div>
            <div class="chart-bar-bg">
                <div class="chart-bar-fill" style="width: ${percent}%;"></div>
            </div>
            <div class="chart-value">${t.points}</div>
        `;
        container.appendChild(row);
    });
}
