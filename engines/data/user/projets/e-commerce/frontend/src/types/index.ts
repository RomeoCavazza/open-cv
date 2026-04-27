export interface Product {
    id: string;
    name: string;
    slug: string;
    price: number;
    description: string;
    features: string[];
    imageUrls: string[];
    category: string;
    stock?: number;
}
