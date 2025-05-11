import { Body, Controller, Post } from '@nestjs/common';
import { AddProductRequestDto } from '@shared';

@Controller({
    path: '/products',
    version: '1',
})
export class AddProductController {
    constructor(private readonly handler: AddProductHandler) {}

    @Post()
    async addProduct(
        @Body() createProductRequest: AddProductRequestDto
    ): Promise<ProductResponseDto> {
        const product = await this.handler.addProduct(createProductRequest);
        return product;
    }
}
