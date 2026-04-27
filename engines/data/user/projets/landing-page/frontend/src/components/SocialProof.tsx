import { Instagram } from "lucide-react";
import testimonial1 from "@/assets/testimonial-1.jpg";
import testimonial2 from "@/assets/testimonial-2.jpg";
import testimonial3 from "@/assets/testimonial-3.jpg";

interface Testimonial {
  id: number;
  image: string;
  username: string;
  text: string;
}

const testimonials: Testimonial[] = [
  {
    id: 1,
    image: testimonial1,
    username: "@luna_the_cat",
    text: "Enfin des croquettes aussi belles que ma déco ! 🏠✨",
  },
  {
    id: 2,
    image: testimonial2,
    username: "@aesthetic_paws",
    text: "Mon chat est obsédé et mes amis aussi ! 😍",
  },
  {
    id: 3,
    image: testimonial3,
    username: "@cozy_whiskers",
    text: "La qualité premium qu'on attendait tous ! 🐱💕",
  },
];

const TestimonialCard = ({
  testimonial,
  index,
}: {
  testimonial: Testimonial;
  index: number;
}) => {
  return (
    <div
      className="opacity-0 animate-fade-in-up group"
      style={{ animationDelay: `${index * 100}ms` }}
    >
      <div className="bg-card rounded-3xl overflow-hidden shadow-soft hover:shadow-card transition-all duration-300 hover:-translate-y-2">
        {/* Image */}
        <div className="relative aspect-square overflow-hidden">
          <img
            src={testimonial.image}
            alt={testimonial.username}
            className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-foreground/60 to-transparent" />
          <div className="absolute bottom-4 left-4 right-4">
            <p className="text-primary-foreground text-sm font-medium mb-1">
              {testimonial.text}
            </p>
          </div>
        </div>
        {/* Username */}
        <div className="p-4 flex items-center gap-2">
          <Instagram className="w-4 h-4 text-primary" />
          <span className="text-sm font-medium text-foreground">
            {testimonial.username}
          </span>
        </div>
      </div>
    </div>
  );
};

const SocialProof = () => {
  return (
    <section id="testimonials" className="py-20 lg:py-32">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center mb-16 opacity-0 animate-fade-in-up">
          <span className="inline-block bg-secondary text-secondary-foreground px-4 py-2 rounded-full text-sm font-medium mb-4">
            Communauté
          </span>
          <h2 className="font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-foreground mb-4">
            Vus chez vous <span className="text-primary">@MeowMeow</span>
          </h2>
          <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
            Rejoignez notre communauté de cat parents esthètes.
          </p>
        </div>

        {/* Testimonial Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto">
          {testimonials.map((testimonial, index) => (
            <TestimonialCard
              key={testimonial.id}
              testimonial={testimonial}
              index={index}
            />
          ))}
        </div>

        {/* CTA */}
        <div className="text-center mt-12 opacity-0 animate-fade-in-up animation-delay-400">
          <a
            href="https://instagram.com"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 text-primary hover:text-primary/80 transition-colors font-medium"
          >
            <Instagram className="w-5 h-5" />
            Suivez-nous sur Instagram
          </a>
        </div>
      </div>
    </section>
  );
};

export default SocialProof;
