export const API_BASE_URL =
    process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5001/api';

export const API_ENDPOINTS = {
    PRODUCTS: `${API_BASE_URL}/products`,
    HEALTH: `${API_BASE_URL}/health`,
};
