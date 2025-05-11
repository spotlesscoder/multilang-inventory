import { Body, Controller, Post } from '@nestjs/common';
import { AddProductRequestDto } from '@shared';
import { AddProductHandler } from './add-product.handler';

@Controller({
    path: '/products',
    version: '1',
})
export class AddProductController {
    constructor(private readonly handler: AddProductHandler) {}

    @Post()
    async addProduct(@Body() dto: AddProductRequestDto): Promise<void> {}
}
