import { Product } from '../../app/products/db/schemas/product.schema';

export function sampleProduct(): Product {
    const entity = new Product();
    entity._id = 'productId';
    entity.name = 'Banana';

    return entity;
}
