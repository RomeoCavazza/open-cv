"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { ShoppingBag, X, Plus, Minus, Trash2 } from 'lucide-react';
import { useCart } from '@/providers/CartProvider';
import { Button } from '@/components/ui/Button';
import { cn } from '@/lib/utils';

export const Header = () => {
  const { items, getItemCount, getTotal, updateQuantity, removeItem } = useCart();
  const [isCartOpen, setIsCartOpen] = useState(false);
  const [isScrolled, setIsScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 50);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  return (
    <>
      <header
        className={cn(
          "fixed top-0 left-0 right-0 z-50 transition-all duration-300 px-6 py-4 flex items-center justify-between",
          isScrolled ? "bg-brand-black/80 backdrop-blur-md border-b border-brand-gray/20" : "bg-transparent"
        )}
      >
        <Link href="/" className="flex items-center">
          <Image
            src="/assets/logo-light.png"
            alt="DRYVIA"
            width={120}
            height={40}
            className="h-8 w-auto object-contain"
          />
        </Link>

        <nav className="hidden md:flex items-center space-x-8">
          <Link href="/" className="text-brand-white hover:text-brand-green transition-colors font-heading uppercase tracking-widest text-sm">
            Home
          </Link>
          <Link href="/shop" className="text-brand-white hover:text-brand-green transition-colors font-heading uppercase tracking-widest text-sm">
            Shop
          </Link>
        </nav>

        <div className="flex items-center space-x-4">
          <button
            onClick={() => setIsCartOpen(true)}
            className="relative p-2 text-brand-white hover:text-brand-green transition-colors"
          >
            <ShoppingBag size={24} />
            {getItemCount() > 0 && (
              <span className="absolute top-0 right-0 bg-brand-green text-brand-black text-[10px] font-bold w-5 h-5 rounded-full flex items-center justify-center">
                {getItemCount()}
              </span>
            )}
          </button>
        </div>
      </header>

      {/* Cart Sidebar / Sheet */}
      <div
        className={cn(
          "fixed inset-0 z-[60] bg-black/60 transition-opacity duration-300",
          isCartOpen ? "opacity-100 pointer-events-auto" : "opacity-0 pointer-events-none"
        )}
        onClick={() => setIsCartOpen(false)}
      />
      <div
        className={cn(
          "fixed top-0 right-0 bottom-0 z-[70] w-full max-w-md bg-brand-black border-l border-brand-gray/20 transition-transform duration-300 ease-in-out transform",
          isCartOpen ? "translate-x-0" : "translate-x-full"
        )}
      >
        <div className="flex flex-col h-full">
          <div className="flex items-center justify-between p-6 border-b border-brand-gray/20">
            <h2 className="text-xl font-heading font-bold uppercase tracking-widest">Your Bag</h2>
            <button onClick={() => setIsCartOpen(false)} className="text-brand-gray hover:text-brand-white transition-colors">
              <X size={24} />
            </button>
          </div>

          <div className="flex-1 overflow-y-auto p-6 space-y-6">
            {items.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-full text-center">
                <ShoppingBag size={48} className="text-brand-gray/30 mb-4" />
                <p className="text-brand-gray">Your bag is empty.</p>
                <Button variant="outline" className="mt-6" onClick={() => setIsCartOpen(false)}>
                  Start Shopping
                </Button>
              </div>
            ) : (
              items.map((item) => (
                <div key={item.id} className="flex space-x-4">
                  <div className="relative w-24 h-24 bg-brand-gray/10 flex-shrink-0">
                    <Image
                      src={item.imageUrl}
                      alt={item.name}
                      fill
                      className="object-contain p-2"
                    />
                  </div>
                  <div className="flex-1 flex flex-col justify-between">
                    <div>
                      <div className="flex justify-between">
                        <h3 className="font-heading font-bold text-sm uppercase">{item.name}</h3>
                        <button onClick={() => removeItem(item.id)} className="text-brand-gray hover:text-red-500 transition-colors">
                          <Trash2 size={16} />
                        </button>
                      </div>
                      <p className="text-brand-green font-bold mt-1">€{item.price.toFixed(2)}</p>
                    </div>
                    <div className="flex items-center space-x-3">
                      <button
                        onClick={() => updateQuantity(item.id, item.quantity - 1)}
                        className="w-8 h-8 flex items-center justify-center border border-brand-gray/30 hover:border-brand-green text-brand-gray hover:text-brand-green transition-all"
                      >
                        <Minus size={14} />
                      </button>
                      <span className="text-sm font-bold w-4 text-center">{item.quantity}</span>
                      <button
                        onClick={() => updateQuantity(item.id, item.quantity + 1)}
                        className="w-8 h-8 flex items-center justify-center border border-brand-gray/30 hover:border-brand-green text-brand-gray hover:text-brand-green transition-all"
                      >
                        <Plus size={14} />
                      </button>
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>

          {items.length > 0 && (
            <div className="p-6 border-t border-brand-gray/20 space-y-4">
              <div className="flex justify-between items-center text-lg font-bold">
                <span className="uppercase font-heading">Total</span>
                <span className="text-brand-green">€{getTotal().toFixed(2)}</span>
              </div>
              <p className="text-xs text-brand-gray text-center">Shipping & taxes calculated at checkout.</p>
              <div className="grid grid-cols-1 gap-3">
                <Link href="/cart" className="w-full">
                  <Button variant="outline" className="w-full" onClick={() => setIsCartOpen(false)}>
                    View Bag
                  </Button>
                </Link>
                <Link href="/checkout" className="w-full">
                  <Button className="w-full" onClick={() => setIsCartOpen(false)}>
                    Checkout
                  </Button>
                </Link>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
};
