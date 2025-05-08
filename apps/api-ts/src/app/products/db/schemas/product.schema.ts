import { Prop, Schema, SchemaFactory } from '@nestjs/mongoose';
import { HydratedDocument } from 'mongoose';
import { id } from '../../../infra/id';

export type ProductDoc = HydratedDocument<Product>;

@Schema({
    collection: 'products',
})
export class Product {
    @Prop({ required: true, default: () => id() })
    _id: string;

    @Prop({ required: true, unique: true })
    name: string;
}

export const ProductSchema = SchemaFactory.createForClass(Product);
