"use client";

import React, { useEffect, useState } from 'react';
import Image from 'next/image';
import { useParams } from 'next/navigation';
import { Button } from '@/components/ui/Button';
import { useCart } from '@/providers/CartProvider';
import { ShoppingBag, ChevronLeft, ChevronRight, CheckCircle2 } from 'lucide-react';
import { API_ENDPOINTS } from '@/config/api.config';
import { Product } from '@/types';
import { initialProducts } from '@/data/products';
import Link from 'next/link';

export default function ProductDetailPage() {
  const { slug } = useParams();

  // Find local fallback first
  const localFallback = initialProducts.find(p => p.slug === slug);

  const [product, setProduct] = useState<Product | null>(localFallback || null);
  const [isLoading, setIsLoading] = useState(true);
  const [activeImage, setActiveImage] = useState(0);
  const { addItem } = useCart();

  useEffect(() => {
    const fetchProduct = async () => {
      try {
        const response = await fetch(`${API_ENDPOINTS.PRODUCTS}/${slug}`);
        if (!response.ok) throw new Error('Product not found');
        const data = await response.json();
        setProduct(data);
      } catch (error) {
        console.error('Error fetching product:', error);
      } finally {
        setIsLoading(false);
      }
    };

    if (slug) fetchProduct();
  }, [slug]);

  if (isLoading) return <div className="pt-32 text-center">Loading...</div>;
  if (!product) return <div className="pt-32 text-center">Product not found.</div>;

  return (
    <div className="pt-32 pb-24 px-6 max-w-7xl mx-auto">
      <Link href="/shop" className="inline-flex items-center text-brand-gray hover:text-brand-green mb-8 transition-colors uppercase text-xs font-bold tracking-widest">
        <ChevronLeft size={16} className="mr-1" /> Back to Shop
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-16">
        {/* Left: Gallery */}
        <div className="space-y-4">
          <div className="relative aspect-square bg-brand-gray/5 overflow-hidden border border-brand-gray/10">
            <Image
              src={product.imageUrls[activeImage]}
              alt={product.name}
              fill
              className="object-contain p-12 transition-all duration-500"
            />

            {/* Gallery Navigation Arrows */}
            <button
              onClick={() => setActiveImage(prev => (prev === 0 ? product.imageUrls.length - 1 : prev - 1))}
              className="absolute left-4 top-1/2 -translate-y-1/2 p-2 bg-brand-black/50 text-brand-white hover:bg-brand-green hover:text-brand-black transition-all"
            >
              <ChevronLeft size={20} />
            </button>
            <button
              onClick={() => setActiveImage(prev => (prev === product.imageUrls.length - 1 ? 0 : prev + 1))}
              className="absolute right-4 top-1/2 -translate-y-1/2 p-2 bg-brand-black/50 text-brand-white hover:bg-brand-green hover:text-brand-black transition-all"
            >
              <ChevronRight size={20} />
            </button>
          </div>

          <div className="grid grid-cols-5 gap-4">
            {product.imageUrls.map((url, idx) => (
              <button
                key={idx}
                onClick={() => setActiveImage(idx)}
                className={`relative aspect-square bg-brand-gray/5 border-2 transition-all overflow-hidden ${activeImage === idx ? 'border-brand-green' : 'border-transparent opacity-60 hover:opacity-100'
                  }`}
              >
                <Image src={url} alt={`${product.name} view ${idx + 1}`} fill className="object-contain p-2" />
              </button>
            ))}
          </div>
        </div>

        {/* Right: Info */}
        <div className="flex flex-col">
          <div className="mb-8">
            <span className="bg-brand-gray/10 text-brand-gray px-3 py-1 text-[10px] font-bold uppercase tracking-widest mb-4 inline-block">
              {product.category}
            </span>
            <h1 className="text-4xl md:text-6xl font-heading font-black uppercase tracking-tighter mb-4">
              {product.name}
            </h1>
            <div className="text-3xl font-heading font-bold text-brand-green">€{product.price.toFixed(2)}</div>
          </div>

          <p className="text-brand-gray text-lg leading-relaxed mb-8">
            {product.description}
          </p>

          <div className="space-y-4 mb-10">
            <h3 className="font-heading font-bold uppercase tracking-widest text-sm text-brand-white">Key Features</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              {product.features.map((feature, idx) => (
                <div key={idx} className="flex items-start space-x-2">
                  <CheckCircle2 size={18} className="text-brand-green mt-0.5 flex-shrink-0" />
                  <span className="text-sm text-brand-gray">{feature}</span>
                </div>
              ))}
            </div>
          </div>

          <div className="mt-auto pt-8 border-t border-brand-gray/20">
            <Button
              size="lg"
              className="w-full sm:w-auto px-12"
              onClick={() => addItem(product)}
            >
              <ShoppingBag size={20} className="mr-2" /> Add to Bag
            </Button>
            <p className="mt-4 text-[10px] text-brand-gray uppercase tracking-widest text-center sm:text-left">
              Free shipping on orders over €150. 30-day free returns.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
