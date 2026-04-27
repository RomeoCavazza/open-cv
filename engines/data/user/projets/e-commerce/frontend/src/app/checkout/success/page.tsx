"use client";

import React from 'react';
import Link from 'next/link';
import { Button } from '@/components/ui/Button';
import { CheckCircle2, ArrowRight } from 'lucide-react';

export default function SuccessPage() {
  const orderNumber = Math.floor(Math.random() * 900000) + 100000;

  return (
    <div className="pt-48 pb-32 px-6 max-w-2xl mx-auto text-center">
      <div className="flex flex-col items-center">
        <div className="w-24 h-24 bg-brand-green/10 rounded-full flex items-center justify-center mb-10 animate-bounce-slow">
          <CheckCircle2 size={48} className="text-brand-green" />
        </div>
        
        <h1 className="text-4xl md:text-7xl font-heading font-black uppercase tracking-tighter mb-4">
          Order <span className="text-brand-green">Confirmed</span>
        </h1>
        
        <p className="text-brand-gray text-lg mb-4 uppercase tracking-widest font-bold">
          Order #{orderNumber}
        </p>
        
        <p className="text-brand-gray mb-12 max-w-md mx-auto leading-relaxed">
          Thank you for choosing DRYVIA. We've received your order and are preparing your performance gear for shipment. You'll receive a confirmation email shortly.
        </p>
        
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full">
          <Link href="/shop" className="w-full">
            <Button variant="outline" size="lg" className="w-full">
              Shop More
            </Button>
          </Link>
          <Link href="/" className="w-full">
            <Button size="lg" className="w-full">
              Home <ArrowRight size={20} className="ml-2" />
            </Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
