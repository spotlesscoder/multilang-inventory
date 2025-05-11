import { z } from 'zod';

export type AddProductRequestDto = z.infer<typeof addProductRequestSchema>;

export const addProductRequestSchema = z.strictObject({
    category: z.string().min(1),
    name: z.string().min(1),
});
