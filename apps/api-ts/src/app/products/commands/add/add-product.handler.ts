import { Injectable } from '@nestjs/common';
import { ProductsRepository } from '../../db/products.repository';

@Injectable()
export class AddProductHandler {
    constructor(private readonly productsRepository: ProductsRepository) {}
}
