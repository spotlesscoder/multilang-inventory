import { Injectable, Logger } from '@nestjs/common';
import { ProductsRepository } from './db/products.repository';
import { Product } from './db/schemas/product.schema';

@Injectable()
export class ProductsService {
    private readonly logger = new Logger(ProductsService.name);

    constructor(private readonly repository: ProductsRepository) {}

    async create(product: Product) {
        this.logger.log('Creating product');
        return this.repository.create(product);
    }
}
