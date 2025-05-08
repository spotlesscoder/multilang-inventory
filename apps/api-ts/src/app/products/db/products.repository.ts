import { Injectable } from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { DeleteResult, Model, UpdateWriteOpResult } from 'mongoose';
import { Product, ProductDoc } from './schemas/product.schema';

@Injectable()
export class ProductsRepository {
    constructor(
        @InjectModel(Product.name) private readonly model: Model<ProductDoc>
    ) {}

    create(product: Product): Promise<Product> {
        return this.model.create(product);
    }

    addEAN(_id: string, ean: string): Promise<UpdateWriteOpResult> {
        return this.model.updateMany({ _id }, { $push: { eans: [ean] } });
    }

    delete(_id: string): Promise<DeleteResult> {
        return this.model.deleteMany({ _id });
    }
}
