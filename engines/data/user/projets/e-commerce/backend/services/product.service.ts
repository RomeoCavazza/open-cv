import { Product, initialProducts } from '../models/product.model';

export class ProductService {
  private products: Product[] = initialProducts;

  async getAllProducts(): Promise<Product[]> {
    return this.products;
  }

  async getProductBySlug(slug: string): Promise<Product | undefined> {
    return this.products.find(p => p.slug === slug);
  }

  async getProductById(id: string): Promise<Product | undefined> {
    return this.products.find(p => p.id === id);
  }
}

export const productService = new ProductService();
