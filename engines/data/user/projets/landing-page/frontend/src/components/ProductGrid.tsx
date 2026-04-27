import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import packagingCollection from "@/assets/packaging-collection.png";

interface Product {
  id: number;
  name: string;
  subtitle: string;
  price: string;
  accentColor: string;
  accentBg: string;
  image: string;
}

const products: Product[] = [
  {
    id: 1,
    name: "Original Premium Blend",
    subtitle: "La recette signature",
    price: "24.90€",
    accentColor: "text-primary",
    accentBg: "bg-primary/10",
    image: "/packaging-front.png",
  },
  {
    id: 2,
    name: "Silky Coat Blend",
    subtitle: "Pour un pelage soyeux",
    price: "24.90€",
    accentColor: "text-golden-beige",
    accentBg: "bg-golden-beige/20",
    image: "/packaging-poil-soyeux.png",
  },
  {
    id: 3,
    name: "Salmon Supreme Blend",
    subtitle: "Riche en Oméga-3",
    price: "24.90€",
    accentColor: "text-soft-rose",
    accentBg: "bg-secondary",
    image: "/packaging-saumon.png",
  },
  {
    id: 4,
    name: "Vitamins Boost Blend",
    subtitle: "Vitalité optimale",
    price: "24.90€",
    accentColor: "text-matcha-green",
    accentBg: "bg-matcha-green/20",
    image: "/packaging-vitamines.png",
  },
];

const ProductCard = ({ product, index }: { product: Product; index: number }) => {
  return (
    <div
      className={`opacity-0 animate-fade-in-up group`}
      style={{ animationDelay: `${index * 100}ms` }}
    >
      <div className="bg-card rounded-3xl p-6 shadow-soft hover:shadow-card transition-all duration-300 hover:-translate-y-2">
        {/* Product Image */}
        <div className={`${product.accentBg} rounded-2xl aspect-square mb-4 flex items-center justify-center overflow-hidden relative`}>
          <img
            src={product.image}
            alt={product.name}
            className="w-full h-full object-contain p-4"
          />
          <div className={`absolute top-3 right-3 ${product.accentBg} rounded-full px-3 py-1`}>
            <span className={`text-xs font-semibold ${product.accentColor}`}>Nouveau</span>
          </div>
        </div>
        
        {/* Product Info */}
        <div className="space-y-2">
          <h3 className="font-heading font-bold text-lg text-foreground group-hover:text-primary transition-colors">
            {product.name}
          </h3>
          <p className="text-sm text-muted-foreground">{product.subtitle}</p>
          <div className="flex items-center justify-between pt-2">
            <span className="font-heading font-bold text-xl text-foreground">
              {product.price}
            </span>
            <Button variant="add" size="icon" className="rounded-xl">
              <Plus className="w-5 h-5" />
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};

const ProductGrid = () => {
  return (
    <section id="collection" className="py-20 lg:py-32">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center mb-16 opacity-0 animate-fade-in-up">
          <span className="inline-block bg-secondary text-secondary-foreground px-4 py-2 rounded-full text-sm font-medium mb-4">
            La Collection
          </span>
          <h2 className="font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-foreground mb-4">
            Nos recettes <span className="text-primary">premium</span>
          </h2>
          <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
            Des ingrédients sélectionnés avec soin, dans un packaging design qui sublime votre intérieur.
          </p>
        </div>

        {/* Hero Collection Image */}
        <div className="mb-16 opacity-0 animate-fade-in-up animation-delay-100">
          <img
            src={packagingCollection}
            alt="Collection complète Meow Meow"
            className="w-full max-w-4xl mx-auto rounded-3xl shadow-card"
          />
        </div>

        {/* Product Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          {products.map((product, index) => (
            <ProductCard key={product.id} product={product} index={index} />
          ))}
        </div>
      </div>
    </section>
  );
};

export default ProductGrid;
