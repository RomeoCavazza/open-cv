import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Instagram, Facebook, Twitter } from "lucide-react";

const Footer = () => {
  return (
    <footer className="bg-dark-roast text-creamy-latte py-16 lg:py-20">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-12 mb-12">
          {/* Brand */}
          <div className="lg:col-span-2">
            <img
              src="/logo-light.png"
              alt="Meow Meow"
              className="w-16 h-16 mb-4"
            />
            <p className="text-creamy-latte/80 max-w-md mb-6">
              Des croquettes premium pour chat, dans un packaging design qui sublime votre intérieur. 
              Rejoignez la révolution féline.
            </p>
            {/* Newsletter */}
            <div className="flex gap-3 max-w-md">
              <Input
                type="email"
                placeholder="votre@email.com"
                className="bg-creamy-latte/10 border-creamy-latte/20 text-creamy-latte placeholder:text-creamy-latte/50 rounded-xl focus:border-primary"
              />
              <Button variant="cta" size="pill" className="shrink-0">
                S'inscrire
              </Button>
            </div>
          </div>

          {/* Links */}
          <div>
            <h4 className="font-heading font-bold text-lg mb-4">Navigation</h4>
            <ul className="space-y-3">
              <li>
                <a href="#collection" className="text-creamy-latte/80 hover:text-creamy-latte transition-colors">
                  Nos Recettes
                </a>
              </li>
              <li>
                <a href="#why" className="text-creamy-latte/80 hover:text-creamy-latte transition-colors">
                  Philosophie
                </a>
              </li>
              <li>
                <a href="#testimonials" className="text-creamy-latte/80 hover:text-creamy-latte transition-colors">
                  Avis
                </a>
              </li>
              <li>
                <a href="#" className="text-creamy-latte/80 hover:text-creamy-latte transition-colors">
                  FAQ
                </a>
              </li>
            </ul>
          </div>

          {/* Contact */}
          <div>
            <h4 className="font-heading font-bold text-lg mb-4">Contact</h4>
            <ul className="space-y-3">
              <li>
                <a href="mailto:hello@meowmeow.com" className="text-creamy-latte/80 hover:text-creamy-latte transition-colors">
                  hello@meowmeow.com
                </a>
              </li>
              <li className="text-creamy-latte/80">
                Paris, France 🇫🇷
              </li>
            </ul>
            {/* Social Links */}
            <div className="flex gap-4 mt-6">
              <a
                href="#"
                className="w-10 h-10 bg-creamy-latte/10 rounded-full flex items-center justify-center hover:bg-primary transition-colors"
              >
                <Instagram className="w-5 h-5" />
              </a>
              <a
                href="#"
                className="w-10 h-10 bg-creamy-latte/10 rounded-full flex items-center justify-center hover:bg-primary transition-colors"
              >
                <Facebook className="w-5 h-5" />
              </a>
              <a
                href="#"
                className="w-10 h-10 bg-creamy-latte/10 rounded-full flex items-center justify-center hover:bg-primary transition-colors"
              >
                <Twitter className="w-5 h-5" />
              </a>
            </div>
          </div>
        </div>

        {/* Copyright */}
        <div className="pt-8 border-t border-creamy-latte/20">
          <div className="flex flex-col md:flex-row justify-between items-center gap-4">
            <p className="text-creamy-latte/60 text-sm">
              © 2026 Meow Meow. Tous droits réservés.
            </p>
            <div className="flex gap-6 text-sm">
              <a href="#" className="text-creamy-latte/60 hover:text-creamy-latte transition-colors">
                Mentions légales
              </a>
              <a href="#" className="text-creamy-latte/60 hover:text-creamy-latte transition-colors">
                CGV
              </a>
              <a href="#" className="text-creamy-latte/60 hover:text-creamy-latte transition-colors">
                Confidentialité
              </a>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
