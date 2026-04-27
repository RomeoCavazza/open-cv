export interface Product {
  id: string;
  name: string;
  slug: string;
  price: number;
  description: string;
  features: string[];
  imageUrls: string[];
  category: string;
  stock: number;
}

export const initialProducts: Product[] = [
  {
    id: "1",
    name: "DRYVIA One",
    slug: "dryvia-one",
    price: 129,
    description: "The first indoor anti-sweat sneaker designed for high-intensity training. Features our proprietary Flash-Dry Mesh and Anti-Transfer Sole technology.",
    features: [
      "Anti-Transfer Sole: Leaves no marks on gym floors",
      "Flash-Dry Mesh: Ultimate breathability and moisture wicking",
      "Hygiene Shield: Anti-bacterial lining prevents odors",
      "Eco-Impact: 100% vegan and recycled materials"
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
