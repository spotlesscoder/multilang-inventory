import { z } from 'zod';
export type AddProductRequestDto = z.infer<typeof addProductRequestSchema>;
export declare const addProductRequestSchema: z.ZodObject<{
    category: z.ZodString;
    name: z.ZodString;
}, "strict", z.ZodTypeAny, {
    category: string;
    name: string;
}, {
    category: string;
    name: string;
}>;
//# sourceMappingURL=add-product.request.schema.d.ts.map