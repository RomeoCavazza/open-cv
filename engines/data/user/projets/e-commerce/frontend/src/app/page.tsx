import React from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { Button } from '@/components/ui/Button';
import { Card, CardContent } from '@/components/ui/Card';
import { ArrowRight, Shield, Zap, Wind, Droplets } from 'lucide-react';

export default function Home() {
  return (
    <div className="flex flex-col w-full">
      {/* Hero Section */}
      <section className="relative h-screen w-full flex items-center justify-center overflow-hidden">
        <video
          autoPlay
          muted
          loop
          playsInline
          className="absolute inset-0 w-full h-full object-cover"
        >
          <source src="/assets/hero.mp4" type="video/mp4" />
          {/* Fallback image if video fails */}
          <img src="/assets/hero-banner.png" alt="DRYVIA Hero" className="absolute inset-0 w-full h-full object-cover" />
        </video>
        
        {/* Overlay */}
        <div className="absolute inset-0 bg-brand-black/60 z-10" />

        {/* Hero Content */}
        <div className="relative z-20 text-center px-6 max-w-5xl mx-auto">
          <h1 className="text-5xl md:text-8xl font-heading font-black text-brand-white uppercase tracking-tighter mb-6 animate-fade-in">
            Stay Dry. <br />
            <span className="text-brand-green">Train Hard.</span>
          </h1>
          <p className="text-xl md:text-2xl text-brand-gray mb-10 max-w-2xl mx-auto font-light">
            The first indoor anti-sweat sneaker. Engineered for high-intensity performance and absolute hygiene.
          </p>
          <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
            <Link href="/shop">
              <Button size="lg" className="min-w-[200px]">
                Shop Collection
              </Button>
            </Link>
            <Link href="/shop/dryvia-one">
              <Button size="lg" variant="secondary" className="min-w-[200px]">
                Explore DRYVIA One
              </Button>
            </Link>
          </div>
        </div>

        {/* Scroll Indicator */}
        <div className="absolute bottom-10 left-1/2 -translate-x-1/2 z-20 animate-bounce">
          <div className="w-6 h-10 border-2 border-brand-white/30 rounded-full flex justify-center p-1">
            <div className="w-1 h-2 bg-brand-green rounded-full" />
          </div>
        </div>
      </section>

      {/* USP / Features Section */}
      <section className="py-24 px-6 bg-brand-black">
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-5xl font-heading font-bold uppercase tracking-widest mb-4">
              Engineered for <span className="text-brand-green">Hygiene</span>
            </h2>
            <p className="text-brand-gray max-w-2xl mx-auto">
              Our proprietary technology ensures your feet stay cool and your gym stays clean.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            <FeatureCard
              icon={<Shield className="text-brand-green" size={32} />}
              title="Anti-Transfer Sole"
              description="Proprietary rubber compound that leaves zero marks on any indoor surface."
            />
            <FeatureCard
              icon={<Wind className="text-brand-green" size={32} />}
              title="Flash-Dry Mesh"
              description="Advanced dual-layer mesh that expels heat and wicks moisture in seconds."
            />
            <FeatureCard
              icon={<Droplets className="text-brand-green" size={32} />}
              title="Hygiene Shield"
              description="Anti-microbial silver-ion lining that eliminates 99.9% of odor-causing bacteria."
            />
            <FeatureCard
              icon={<Zap className="text-brand-green" size={32} />}
              title="Eco-Impact"
              description="100% vegan construction using recycled ocean plastics and bio-based foams."
            />
          </div>
        </div>
      </section>

      {/* Featured Product Section */}
      <section className="py-24 px-6 bg-white text-brand-black overflow-hidden">
        <div className="max-w-7xl mx-auto flex flex-col lg:flex-row items-center gap-16">
          <div className="lg:w-1/2 relative group">
            <div className="absolute -inset-4 bg-brand-green/20 rounded-full blur-3xl opacity-0 group-hover:opacity-100 transition-opacity duration-700" />
            <Image
              src="/assets/angle-front.png"
              alt="DRYVIA One"
              width={800}
              height={800}
              className="relative z-10 w-full h-auto object-contain transform group-hover:scale-105 transition-transform duration-700"
            />
          </div>
          <div className="lg:w-1/2 space-y-8">
            <div className="inline-block bg-brand-black text-brand-green px-4 py-1 text-xs font-bold uppercase tracking-[0.2em]">
              Featured Product
            </div>
            <h2 className="text-5xl md:text-7xl font-heading font-black uppercase leading-none">
              DRYVIA <br />
              <span className="text-brand-green">One</span>
            </h2>
            <p className="text-xl text-brand-gray leading-relaxed">
              The ultimate indoor training shoe. Lightweight, breathable, and marking-free. Designed for those who demand performance and purity.
            </p>
            <div className="text-4xl font-heading font-bold">€129.00</div>
            <div className="flex flex-col sm:flex-row gap-4">
              <Link href="/shop/dryvia-one">
                <Button size="lg" className="w-full sm:w-auto">
                  Buy Now <ArrowRight className="ml-2" size={20} />
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Lifestyle Section */}
      <section className="relative h-[80vh] w-full flex items-center overflow-hidden">
        <Image
          src="/assets/gym-lifestyle.png"
          alt="DRYVIA Lifestyle"
          fill
          className="object-cover"
        />
        <div className="absolute inset-0 bg-gradient-to-r from-brand-black via-brand-black/40 to-transparent z-10" />
        
        <div className="relative z-20 px-6 md:px-24 max-w-3xl">
          <h2 className="text-4xl md:text-6xl font-heading font-black text-brand-white uppercase mb-6 leading-tight">
            Designed for <br />
            <span className="text-brand-green">The Gym.</span>
          </h2>
          <p className="text-lg text-brand-white/80 mb-8 max-w-lg">
            Whether it's HIIT, heavy lifting, or cross-training, DRYVIA One provides the stability and breathability you need to push your limits.
          </p>
          <Link href="/shop">
            <Button variant="outline" size="lg">
              Explore Performance
            </Button>
          </Link>
        </div>
      </section>

      {/* Tech Section */}
      <section className="py-24 px-6 bg-brand-black">
        <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
          <div className="order-2 lg:order-1 space-y-8">
            <h2 className="text-3xl md:text-5xl font-heading font-bold uppercase tracking-tight">
              Materials <br />
              <span className="text-brand-green">From the Future</span>
            </h2>
            <div className="space-y-6">
              <TechDetail
                title="Flash-Dry Mesh"
                description="Our proprietary weave allows for maximum airflow while maintaining structural integrity during lateral movements."
              />
              <TechDetail
                title="Eco-Foam Midsole"
                description="Responsive cushioning made from carbon-captured bio-polymers. High energy return with low environmental footprint."
              />
              <TechDetail
                title="Zero-Mark Outsole"
                description="Tested on premium wood and synthetic floors. Maximum grip without the residue."
              />
            </div>
          </div>
          <div className="order-1 lg:order-2 relative aspect-square overflow-hidden border border-brand-gray/20">
            <Image
              src="/assets/tech-mesh.png"
              alt="Technology Mesh"
              fill
              className="object-cover hover:scale-110 transition-transform duration-1000"
            />
          </div>
        </div>
      </section>
    </div>
  );
}

function FeatureCard({ icon, title, description }: { icon: React.ReactNode, title: string, description: string }) {
  return (
    <Card className="h-full group">
      <CardContent className="pt-8 flex flex-col items-center text-center">
        <div className="mb-6 transform group-hover:scale-110 transition-transform duration-300">
          {icon}
        </div>
        <h3 className="text-xl font-heading font-bold uppercase mb-4 tracking-wide">{title}</h3>
        <p className="text-brand-gray text-sm leading-relaxed">
          {description}
        </p>
      </CardContent>
    </Card>
  );
}

function TechDetail({ title, description }: { title: string, description: string }) {
  return (
    <div className="space-y-2 border-l-2 border-brand-gray/20 pl-6 hover:border-brand-green transition-colors duration-300">
      <h4 className="text-xl font-heading font-bold uppercase tracking-wide text-brand-white">{title}</h4>
      <p className="text-brand-gray text-sm">{description}</p>
    </div>
  );
}
