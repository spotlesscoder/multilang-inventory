import { Module } from '@nestjs/common';
import { MongooseModule } from '@nestjs/mongoose';
import { ProductsRepository } from './db/products.repository';
import { ProductModel } from './db/schemas/product.schema';
import { ProductsService } from './products.service';

@Module({
    exports: [ProductsService],
    imports: [MongooseModule.forFeature([ProductModel])],
    providers: [ProductsRepository, ProductsService],
})
export class ProductsModule {}
