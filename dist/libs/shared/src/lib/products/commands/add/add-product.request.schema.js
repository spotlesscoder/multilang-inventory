import { z } from 'zod';
export const addProductRequestSchema = z.strictObject({
    category: z.string().min(1),
    name: z.string().min(1),
});
//# sourceMappingURL=add-product.request.schema.js.map