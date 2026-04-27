"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useCart } from '@/providers/CartProvider';
import { Button } from '@/components/ui/Button';
import { Card, CardContent } from '@/components/ui/Card';
import { ChevronLeft, Lock } from 'lucide-react';
import Link from 'next/link';
import Image from 'next/image';

export default function CheckoutPage() {
  const { items, getTotal, clearCart } = useCart();
  const router = useRouter();
  const [isSubmitting, setIsSubmitting] = useState(false);

  const [formData, setFormData] = useState({
    email: '',
    firstName: '',
    lastName: '',
    address: '',
    city: '',
    zipCode: '',
    country: 'France',
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    // Mock order submission
    setTimeout(() => {
      clearCart();
      router.push('/checkout/success');
    }, 1500);
  };

  if (items.length === 0) {
    return (
      <div className="pt-48 pb-32 px-6 text-center">
        <h1 className="text-4xl font-heading font-black uppercase mb-8">Empty Checkout</h1>
        <Link href="/shop">
          <Button>Back to Shop</Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="pt-32 pb-24 px-6 max-w-7xl mx-auto">
      <Link href="/cart" className="inline-flex items-center text-brand-gray hover:text-brand-green mb-8 transition-colors uppercase text-xs font-bold tracking-widest">
        <ChevronLeft size={16} className="mr-1" /> Back to Bag
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-16">
        {/* Left: Form */}
        <div className="space-y-10">
          <div>
            <h1 className="text-4xl font-heading font-black uppercase tracking-tighter mb-2">Checkout</h1>
            <p className="text-brand-gray text-sm uppercase tracking-widest font-bold">Secure Payment</p>
          </div>

          <form onSubmit={handleSubmit} className="space-y-8">
            <div className="space-y-4">
              <h2 className="text-lg font-heading font-bold uppercase tracking-widest">Contact Information</h2>
              <input
                required
                type="email"
                name="email"
                placeholder="Email Address"
                value={formData.email}
                onChange={handleChange}
                className="w-full bg-brand-gray/5 border border-brand-gray/20 px-4 py-3 text-brand-white focus:outline-none focus:border-brand-green transition-colors"
              />
            </div>

            <div className="space-y-4">
              <h2 className="text-lg font-heading font-bold uppercase tracking-widest">Shipping Address</h2>
              <div className="grid grid-cols-2 gap-4">
                <input
                  required
                  type="text"
                  name="firstName"
                  placeholder="First Name"
                  value={formData.firstName}
                  onChange={handleChange}
                  className="w-full bg-brand-gray/5 border border-brand-gray/20 px-4 py-3 text-brand-white focus:outline-none focus:border-brand-green transition-colors"
                />
                <input
                  required
                  type="text"
                  name="lastName"
                  placeholder="Last Name"
                  value={formData.lastName}
                  onChange={handleChange}
                  className="w-full bg-brand-gray/5 border border-brand-gray/20 px-4 py-3 text-brand-white focus:outline-none focus:border-brand-green transition-colors"
                />
              </div>
              <input
                required
                type="text"
                name="address"
                placeholder="Address"
                value={formData.address}
                onChange={handleChange}
                className="w-full bg-brand-gray/5 border border-brand-gray/20 px-4 py-3 text-brand-white focus:outline-none focus:border-brand-green transition-colors"
              />
              <div className="grid grid-cols-2 gap-4">
                <input
                  required
                  type="text"
                  name="city"
                  placeholder="City"
                  value={formData.city}
                  onChange={handleChange}
                  className="w-full bg-brand-gray/5 border border-brand-gray/20 px-4 py-3 text-brand-white focus:outline-none focus:border-brand-green transition-colors"
                />
                <input
                  required
                  type="text"
                  name="zipCode"
                  placeholder="Zip Code"
                  value={formData.zipCode}
                  onChange={handleChange}
                  className="w-full bg-brand-gray/5 border border-brand-gray/20 px-4 py-3 text-brand-white focus:outline-none focus:border-brand-green transition-colors"
                />
              </div>
            </div>

            <Button size="lg" className="w-full" disabled={isSubmitting}>
              {isSubmitting ? 'Processing...' : (
                <span className="flex items-center">
                  <Lock size={18} className="mr-2" /> Pay €{getTotal().toFixed(2)}
                </span>
              )}
            </Button>
          </form>
        </div>

        {/* Right: Summary */}
        <div className="lg:pl-16">
          <div className="sticky top-32 space-y-8">
            <h2 className="text-xl font-heading font-bold uppercase tracking-widest border-b border-brand-gray/10 pb-4">
              Your Order
            </h2>
            <div className="space-y-6 max-h-[40vh] overflow-y-auto pr-4">
              {items.map((item) => (
                <div key={item.id} className="flex items-center space-x-4">
                  <div className="relative w-16 h-16 bg-brand-gray/5 flex-shrink-0 border border-brand-gray/10">
                    <Image src={item.imageUrl} alt={item.name} fill className="object-contain p-2" />
                    <span className="absolute -top-2 -right-2 bg-brand-green text-brand-black text-[10px] font-bold w-5 h-5 rounded-full flex items-center justify-center">
                      {item.quantity}
                    </span>
                  </div>
                  <div className="flex-1">
                    <h3 className="text-sm font-heading font-bold uppercase">{item.name}</h3>
                    <p className="text-brand-gray text-xs uppercase tracking-widest">DRYVIA Performance</p>
                  </div>
                  <div className="text-sm font-bold">€{(item.price * item.quantity).toFixed(2)}</div>
                </div>
              ))}
            </div>

            <div className="space-y-4 pt-6 border-t border-brand-gray/10">
              <div className="flex justify-between text-brand-gray text-sm">
                <span>Subtotal</span>
                <span>€{getTotal().toFixed(2)}</span>
              </div>
              <div className="flex justify-between text-brand-gray text-sm">
                <span>Shipping</span>
                <span className="text-brand-green uppercase font-bold text-[10px]">Free</span>
              </div>
              <div className="flex justify-between items-center text-xl font-bold pt-4">
                <span className="uppercase font-heading">Total</span>
                <span className="text-brand-green">€{getTotal().toFixed(2)}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
