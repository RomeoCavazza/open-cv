"use client";

import React, { useEffect, useState } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { Button } from '@/components/ui/Button';
import { Card, CardContent, CardFooter } from '@/components/ui/Card';
import { ShoppingBag } from 'lucide-react';
import { useCart } from '@/providers/CartProvider';
import { API_ENDPOINTS } from '@/config/api.config';
import { Product } from '@/types';
import { initialProducts } from '@/data/products';

export default function ShopPage() {
  const [products, setProducts] = useState<Product[]>(initialProducts);
  const [isLoading, setIsLoading] = useState(true);
  const { addItem } = useCart();

  useEffect(() => {
    const fetchProducts = async () => {
      try {
        const response = await fetch(API_ENDPOINTS.PRODUCTS);
        const data = await response.json();
        setProducts(data);
      } catch (error) {
        console.error('Error fetching products:', error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchProducts();
  }, []);

  return (
    <div className="pt-32 pb-24 px-6 max-w-7xl mx-auto">
      <div className="flex flex-col md:flex-row md:items-end justify-between mb-12 gap-6">
        <div>
          <h1 className="text-4xl md:text-6xl font-heading font-black uppercase tracking-tighter mb-4">
            The <span className="text-brand-green">Collection</span>
          </h1>
          <p className="text-brand-gray max-w-xl">
            High-performance indoor footwear designed for maximum hygiene and zero floor marks.
          </p>
        </div>
        <div className="flex items-center space-x-4 text-sm font-bold uppercase tracking-widest text-brand-gray">
          <span>{products.length} Products</span>
        </div>
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {[1, 2, 3].map((i) => (
            <div key={i} className="aspect-[4/5] bg-brand-gray/10 animate-pulse" />
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {products.map((product) => (
            <Card key={product.id} className="flex flex-col group h-full">
              <Link href={`/shop/${product.slug}`} className="relative aspect-[4/5] overflow-hidden bg-brand-gray/5">
                <Image
                  src={product.imageUrls[0]}
                  alt={product.name}
                  fill
                  className="object-contain p-8 group-hover:scale-110 transition-transform duration-500"
                />
                <div className="absolute top-4 left-4">
                  <span className="bg-brand-black text-brand-green px-3 py-1 text-[10px] font-bold uppercase tracking-widest border border-brand-green/30">
                    {product.category}
                  </span>
                </div>
              </Link>
              <CardContent className="pt-6 flex-1">
                <Link href={`/shop/${product.slug}`}>
                  <h3 className="text-xl font-heading font-bold uppercase tracking-wide group-hover:text-brand-green transition-colors">
                    {product.name}
                  </h3>
                </Link>
                <p className="text-brand-gray text-sm mt-2 line-clamp-2">
                  {product.description}
                </p>
              </CardContent>
              <CardFooter className="flex items-center justify-between pb-6">
                <span className="text-2xl font-heading font-bold">€{product.price.toFixed(2)}</span>
                <Button
                  size="sm"
                  onClick={() => addItem(product)}
                  className="group/btn"
                >
                  <ShoppingBag size={18} className="mr-2 group-hover/btn:animate-bounce" /> Add
                </Button>
              </CardFooter>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
