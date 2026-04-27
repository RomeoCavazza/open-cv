import { Product } from '../types';

export const initialProducts: Product[] = [
    {
        id: "1",
        name: "DRYVIA One",
        slug: "dryvia-one",
        price: 129,
        description: "The first indoor sneaker engineered to keep your feet, socks, and training mat perfectly dry. Designed for HIIT, cross-training, and studio workouts where hygiene and grip are non-negotiable.",
        features: [
            "Anti-Transfer Sole: Sweat never reaches the floor.",
            "Flash-Dry Mesh: Breathable fabric that evaporates moisture instantly.",
            "Hygiene Shield: Antibacterial membrane to prevent odors.",
            "Eco-Impact: Made from recycled materials."
        ],
        imageUrls: [
            "/assets/angle-front.png",
            "/assets/side-view.png",
            "/assets/back-view.png",
            "/assets/sole-view.png",
            "/assets/tech-mesh.png"
        ],
        category: "Indoor Fitness",
        stock: 50
    }
];
