"use client";

import React from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { useCart } from '@/providers/CartProvider';
import { Button } from '@/components/ui/Button';
import { Plus, Minus, Trash2, ArrowRight, ShoppingBag, ChevronLeft } from 'lucide-react';

export default function CartPage() {
  const { items, updateQuantity, removeItem, getTotal, getItemCount } = useCart();

  if (items.length === 0) {
    return (
      <div className="pt-48 pb-32 px-6 max-w-7xl mx-auto text-center">
        <div className="flex flex-col items-center justify-center">
          <ShoppingBag size={80} className="text-brand-gray/20 mb-8" />
          <h1 className="text-4xl md:text-6xl font-heading font-black uppercase tracking-tighter mb-4">
            Your Bag is <span className="text-brand-green">Empty</span>
          </h1>
          <p className="text-brand-gray mb-12 max-w-md mx-auto">
            You haven't added any DRYVIA products to your bag yet.
          </p>
          <Link href="/shop">
            <Button size="lg">Start Shopping</Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="pt-32 pb-24 px-6 max-w-7xl mx-auto">
      <div className="flex flex-col md:flex-row md:items-end justify-between mb-12 gap-6">
        <div>
          <h1 className="text-4xl md:text-6xl font-heading font-black uppercase tracking-tighter mb-4">
            Shopping <span className="text-brand-green">Bag</span>
          </h1>
          <p className="text-brand-gray uppercase text-xs font-bold tracking-widest">
            {getItemCount()} {getItemCount() === 1 ? 'Item' : 'Items'} in your bag
          </p>
        </div>
        <Link href="/shop" className="inline-flex items-center text-brand-gray hover:text-brand-green transition-colors uppercase text-xs font-bold tracking-widest">
          <ChevronLeft size={16} className="mr-1" /> Continue Shopping
        </Link>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-16">
        {/* Cart Items */}
        <div className="lg:col-span-2 space-y-8">
          {items.map((item) => (
            <div key={item.id} className="flex flex-col sm:flex-row items-start sm:items-center space-y-4 sm:space-y-0 sm:space-x-8 pb-8 border-b border-brand-gray/10">
              <Link href={`/shop/${item.slug}`} className="relative w-full sm:w-32 aspect-square bg-brand-gray/5 flex-shrink-0">
                <Image
                  src={item.imageUrl}
                  alt={item.name}
                  fill
                  className="object-contain p-4"
                />
              </Link>
              <div className="flex-1 space-y-2">
                <Link href={`/shop/${item.slug}`}>
                  <h3 className="text-xl font-heading font-bold uppercase tracking-wide hover:text-brand-green transition-colors">
                    {item.name}
                  </h3>
                </Link>
                <p className="text-brand-gray text-sm">Unit Price: €{item.price.toFixed(2)}</p>
                <div className="flex items-center space-x-6 pt-2">
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
                  <button
                    onClick={() => removeItem(item.id)}
                    className="text-brand-gray hover:text-red-500 transition-colors uppercase text-[10px] font-bold tracking-widest flex items-center"
                  >
                    <Trash2 size={14} className="mr-1" /> Remove
                  </button>
                </div>
              </div>
              <div className="text-right w-full sm:w-auto">
                <div className="text-xl font-heading font-bold">€{(item.price * item.quantity).toFixed(2)}</div>
              </div>
            </div>
          ))}
        </div>

        {/* Summary */}
        <div className="lg:col-span-1">
          <div className="bg-brand-gray/5 border border-brand-gray/10 p-8 space-y-6">
            <h2 className="text-xl font-heading font-bold uppercase tracking-widest border-b border-brand-gray/10 pb-4">
              Order Summary
            </h2>
            <div className="space-y-4">
              <div className="flex justify-between text-brand-gray text-sm">
                <span>Subtotal</span>
                <span>€{getTotal().toFixed(2)}</span>
              </div>
              <div className="flex justify-between text-brand-gray text-sm">
                <span>Shipping</span>
                <span>Calculated at checkout</span>
              </div>
              <div className="flex justify-between text-brand-gray text-sm">
                <span>Estimated Tax</span>
                <span>€0.00</span>
              </div>
            </div>
            <div className="pt-6 border-t border-brand-gray/10 flex justify-between items-center text-xl font-bold">
              <span className="uppercase font-heading">Total</span>
              <span className="text-brand-green">€{getTotal().toFixed(2)}</span>
            </div>
            <Link href="/checkout" className="block w-full pt-4">
              <Button size="lg" className="w-full">
                Checkout <ArrowRight size={20} className="ml-2" />
              </Button>
            </Link>
            <div className="space-y-4 pt-6">
              <p className="text-[10px] text-brand-gray uppercase tracking-widest text-center">
                We accept: Visa, Mastercard, Amex, PayPal
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
