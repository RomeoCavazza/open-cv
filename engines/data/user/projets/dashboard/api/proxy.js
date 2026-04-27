export default async function handler(req, res) {
    const { endpoint } = req.query;
    const API_KEY = process.env.API_KEY;
    const BASE_URL = 'https://api.football-data.org/v4';

    if (!endpoint) {
        return res.status(400).json({ error: 'Endpoint is required' });
    }

    try {
        const response = await fetch(`${BASE_URL}${endpoint}`, {
            headers: {
                'X-Auth-Token': API_KEY,
            },
        });

        if (!response.ok) {
            return res.status(response.status).json({ error: 'API Error' });
        }

        const data = await response.json();
        res.status(200).json(data);
    } catch (error) {
        res.status(500).json({ error: 'Internal Server Error' });
    }
}
