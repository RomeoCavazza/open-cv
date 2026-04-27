import { Button } from "@/components/ui/button";
import { Star } from "lucide-react";
import packagingBanner from "@/assets/packaging-banner.png";

const Hero = () => {
  return (
    <section className="relative min-h-screen flex items-center pt-20 md:pt-0 overflow-hidden">
      {/* Background decoration */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-24 -right-24 w-96 h-96 bg-secondary/50 rounded-full blur-3xl" />
        <div className="absolute bottom-0 -left-24 w-72 h-72 bg-primary/10 rounded-full blur-3xl" />
      </div>

      <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <div className="grid lg:grid-cols-2 gap-12 items-center">
          {/* Text Content */}
          <div className="order-2 lg:order-1">
            <div className="opacity-0 animate-fade-in-up">
              <h1 className="font-heading text-4xl sm:text-5xl lg:text-6xl font-bold text-foreground leading-tight mb-6">
                The first dry food that looks{" "}
                <span className="text-primary">as good</span> as your decoration.
              </h1>
            </div>
            
            <div className="opacity-0 animate-fade-in-up animation-delay-100">
              <p className="text-lg text-muted-foreground mb-8 max-w-lg">
                Des recettes premium, sans céréales, dans un packaging que vous serez fier d'exposer.
              </p>
            </div>
            
            <div className="opacity-0 animate-fade-in-up animation-delay-200 flex flex-col sm:flex-row gap-4 mb-10">
              <Button variant="cta" size="pill-lg">
                Découvrir la collection
              </Button>
              <Button variant="soft" size="pill-lg">
                En savoir plus
              </Button>
            </div>

            {/* Social Proof */}
            <div className="opacity-0 animate-fade-in-up animation-delay-300 flex items-center gap-4">
              <div className="flex -space-x-3">
                {[1, 2, 3, 4].map((i) => (
                  <div
                    key={i}
                    className="w-10 h-10 rounded-full bg-secondary border-2 border-card flex items-center justify-center"
                  >
                    <span className="text-sm">🐱</span>
                  </div>
                ))}
              </div>
              <div>
                <div className="flex items-center gap-1">
                  {[1, 2, 3, 4, 5].map((i) => (
                    <Star
                      key={i}
                      className="w-4 h-4 fill-primary text-primary"
                    />
                  ))}
                </div>
                <p className="text-sm text-muted-foreground">
                  Rejoignez <span className="font-semibold text-foreground">10,000</span> Aesthetic Cat Parents
                </p>
              </div>
            </div>
          </div>

          {/* Image */}
          <div className="order-1 lg:order-2">
            <div className="opacity-0 animate-fade-in-up animation-delay-100 relative">
              <div className="relative z-10">
                <img
                  src={packagingBanner}
                  alt="Meow Meow Premium Cat Food Collection"
                  className="w-full h-auto rounded-3xl shadow-card animate-float"
                />
              </div>
              {/* Decorative elements */}
              <div className="absolute -bottom-6 -right-6 w-24 h-24 bg-secondary rounded-2xl -z-10" />
              <div className="absolute -top-6 -left-6 w-16 h-16 bg-primary/20 rounded-xl -z-10" />
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;
