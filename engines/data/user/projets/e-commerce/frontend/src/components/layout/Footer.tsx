import React from 'react';
import Link from 'next/link';
import Image from 'next/image';

export const Footer = () => {
  return (
    <footer className="bg-brand-black border-t border-brand-gray/20 pt-16 pb-8 px-6">
      <div className="max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-12">
        {/* Branding Column */}
        <div className="space-y-6">
          <Link href="/">
            <Image
              src="/assets/logo-light.png"
              alt="DRYVIA"
              width={140}
              height={50}
              className="h-10 w-auto object-contain"
            />
          </Link>
          <p className="text-brand-gray text-sm leading-relaxed max-w-xs">
            The first indoor anti-sweat sneaker. Engineered for hygiene, performance, and the planet.
          </p>
          <div className="flex space-x-4">
            {/* Social icons placeholders */}
            <div className="w-8 h-8 rounded-full border border-brand-gray/30 flex items-center justify-center text-brand-gray hover:border-brand-green hover:text-brand-green transition-all cursor-pointer">
              <span className="text-xs font-bold">IG</span>
            </div>
            <div className="w-8 h-8 rounded-full border border-brand-gray/30 flex items-center justify-center text-brand-gray hover:border-brand-green hover:text-brand-green transition-all cursor-pointer">
              <span className="text-xs font-bold">TW</span>
            </div>
          </div>
        </div>

        {/* Shop Column */}
        <div className="space-y-6">
          <h3 className="text-brand-white font-heading font-bold uppercase tracking-widest text-sm">Shop</h3>
          <ul className="space-y-4">
            <li>
              <Link href="/shop" className="text-brand-gray hover:text-brand-green transition-colors text-sm">
                All Products
              </Link>
            </li>
            <li>
              <Link href="/shop/dryvia-one" className="text-brand-gray hover:text-brand-green transition-colors text-sm">
                DRYVIA One
              </Link>
            </li>
            <li>
              <Link href="/shop" className="text-brand-gray hover:text-brand-green transition-colors text-sm">
                New Arrivals
              </Link>
            </li>
          </ul>
        </div>

        {/* Company Column */}
        <div className="space-y-6">
          <h3 className="text-brand-white font-heading font-bold uppercase tracking-widest text-sm">Company</h3>
          <ul className="space-y-4">
            <li>
              <Link href="#" className="text-brand-gray hover:text-brand-green transition-colors text-sm">
                About Us
              </Link>
            </li>
            <li>
              <Link href="#" className="text-brand-gray hover:text-brand-green transition-colors text-sm">
                Sustainability
              </Link>
            </li>
            <li>
              <Link href="#" className="text-brand-gray hover:text-brand-green transition-colors text-sm">
                Contact
              </Link>
            </li>
          </ul>
        </div>

        {/* Newsletter Column */}
        <div className="space-y-6">
          <h3 className="text-brand-white font-heading font-bold uppercase tracking-widest text-sm">Newsletter</h3>
          <p className="text-brand-gray text-sm">
            Join the community for exclusive drops and fitness tips.
          </p>
          <form className="flex">
            <input
              type="email"
              placeholder="Your email"
              className="bg-brand-gray/10 border border-brand-gray/30 px-4 py-2 text-sm text-brand-white focus:outline-none focus:border-brand-green flex-1"
            />
            <button className="bg-brand-green text-brand-black px-4 py-2 text-sm font-bold uppercase tracking-widest hover:bg-brand-green/90 transition-all">
              Join
            </button>
          </form>
        </div>
      </div>

      <div className="max-w-7xl mx-auto mt-16 pt-8 border-t border-brand-gray/10 flex flex-col md:flex-row justify-between items-center gap-4">
        <p className="text-brand-gray text-[10px] uppercase tracking-widest">
          © 2026 DRYVIA. All rights reserved.
        </p>
        <div className="flex space-x-6">
          <Link href="#" className="text-brand-gray hover:text-brand-white text-[10px] uppercase tracking-widest transition-colors">
            Terms of Service
          </Link>
          <Link href="#" className="text-brand-gray hover:text-brand-white text-[10px] uppercase tracking-widest transition-colors">
            Privacy Policy
          </Link>
        </div>
      </div>
    </footer>
  );
};
