import { Home, Leaf, Shield } from "lucide-react";
import macroCroquettes from "@/assets/macro-croquettes.png";

interface Feature {
  icon: React.ReactNode;
  title: string;
  description: string;
}

const features: Feature[] = [
  {
    icon: <Home className="w-8 h-8" />,
    title: "Design-First",
    description: "Un packaging pensé pour sublimer votre intérieur.",
  },
  {
    icon: <Leaf className="w-8 h-8" />,
    title: "Ingrédients Sains",
    description: "Sans céréales, sans OGM, que du naturel.",
  },
  {
    icon: <Shield className="w-8 h-8" />,
    title: "Approuvé par les Vétos",
    description: "Formulé avec des experts en nutrition féline.",
  },
];

const WhySection = () => {
  return (
    <section id="why" className="py-20 lg:py-32 bg-card/50">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center mb-16 opacity-0 animate-fade-in-up">
          <span className="inline-block bg-primary/10 text-primary px-4 py-2 rounded-full text-sm font-medium mb-4">
            Philosophie
          </span>
          <h2 className="font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-foreground mb-4">
            Pourquoi <span className="text-primary">Meow Meow</span> ?
          </h2>
          <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
            Parce que votre chat mérite le meilleur, et votre déco aussi.
          </p>
        </div>

        {/* Bento Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto">
          {/* Large Feature Card */}
          <div className="md:col-span-2 lg:col-span-1 lg:row-span-2 opacity-0 animate-fade-in-up">
            <div className="h-full bg-card rounded-3xl p-8 shadow-soft hover:shadow-card transition-all duration-300 flex flex-col">
              <div className="flex-1">
                <div className="w-16 h-16 bg-primary/10 rounded-2xl flex items-center justify-center text-primary mb-6">
                  {features[0].icon}
                </div>
                <h3 className="font-heading font-bold text-2xl text-foreground mb-3">
                  {features[0].title}
                </h3>
                <p className="text-muted-foreground text-lg">
                  {features[0].description}
                </p>
              </div>
              <div className="mt-8">
                <img
                  src={macroCroquettes}
                  alt="Croquettes premium Meow Meow"
                  className="w-full h-48 object-cover rounded-2xl"
                />
              </div>
            </div>
          </div>

          {/* Feature Cards */}
          {features.slice(1).map((feature, index) => (
            <div
              key={feature.title}
              className="opacity-0 animate-fade-in-up"
              style={{ animationDelay: `${(index + 1) * 100}ms` }}
            >
              <div className="h-full bg-card rounded-3xl p-8 shadow-soft hover:shadow-card transition-all duration-300 hover:-translate-y-2">
                <div className="w-14 h-14 bg-secondary rounded-2xl flex items-center justify-center text-foreground mb-6">
                  {feature.icon}
                </div>
                <h3 className="font-heading font-bold text-xl text-foreground mb-3">
                  {feature.title}
                </h3>
                <p className="text-muted-foreground">
                  {feature.description}
                </p>
              </div>
            </div>
          ))}

          {/* Stats Card */}
          <div
            className="opacity-0 animate-fade-in-up"
            style={{ animationDelay: "300ms" }}
          >
            <div className="h-full bg-primary rounded-3xl p-8 shadow-card">
              <div className="grid grid-cols-2 gap-6 h-full">
                <div className="flex flex-col justify-center">
                  <span className="font-heading text-4xl font-bold text-primary-foreground">98%</span>
                  <span className="text-primary-foreground/80 text-sm mt-1">Chats satisfaits</span>
                </div>
                <div className="flex flex-col justify-center">
                  <span className="font-heading text-4xl font-bold text-primary-foreground">100%</span>
                  <span className="text-primary-foreground/80 text-sm mt-1">Naturel</span>
                </div>
                <div className="flex flex-col justify-center">
                  <span className="font-heading text-4xl font-bold text-primary-foreground">0%</span>
                  <span className="text-primary-foreground/80 text-sm mt-1">Céréales</span>
                </div>
                <div className="flex flex-col justify-center">
                  <span className="font-heading text-4xl font-bold text-primary-foreground">5★</span>
                  <span className="text-primary-foreground/80 text-sm mt-1">Note moyenne</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default WhySection;
