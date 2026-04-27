const express = require('express');
const path = require('path');
const fs = require('fs');

const app = express();
const PORT = 3000;

// Serve static files from public directory
app.use(express.static(path.join(__dirname, 'public')));

// Mock API endpoints
// Reads from local JSON files in specific mock directory
const MOCK_DIR = path.join(__dirname, 'mock');

// Helper to read JSON
const readMock = (filename, res) => {
    const filePath = path.join(MOCK_DIR, filename);
    fs.readFile(filePath, 'utf8', (err, data) => {
        if (err) {
            console.error(`Error reading ${filename}:`, err);
            return res.status(500).json({ error: 'Mock data not found' });
        }
        try {
            res.json(JSON.parse(data));
        } catch (parseErr) {
            console.error(`Error parsing ${filename}:`, parseErr);
            res.status(500).json({ error: 'Invalid JSON in mock file' });
        }
    });
};

app.get('/mock/competition', (req, res) => {
    readMock('competition_FL1.json', res);
});

app.get('/mock/standings', (req, res) => {
    readMock('standings_FL1.json', res);
});

app.get('/mock/matches', (req, res) => {
    readMock('matches_FL1.json', res);
});

app.get('/mock/teams', (req, res) => {
    readMock('teams_FL1.json', res);
});

app.listen(PORT, () => {
    console.log(`Server running at http://localhost:${PORT}`);
    console.log(`Serving static files from ./public`);
    console.log(`Mock endpoints enabled at /mock/*`);
});
