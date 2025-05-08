import { Module } from '@nestjs/common';
import { MongooseModule } from '@nestjs/mongoose';
import { ProductsRepository } from './db/products.repository';
import { Product, ProductSchema } from './db/schemas/product.schema';
import { ProductsService } from './products.service';

@Module({
    exports: [ProductsService],
    imports: [
        MongooseModule.forFeature([
            {
                name: Product.name,
                schema: ProductSchema,
            },
        ]),
    ],
    providers: [ProductsRepository, ProductsService],
})
export class ProductsModule {}
