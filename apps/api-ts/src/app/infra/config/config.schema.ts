import z from 'zod';

export const configSchema = z.object({
    REDIS_PORT: z.number().default(6379),
    REDIS_HOST: z.string().default('localhost'),
    REDIS_PASSWORD: z.string().optional(),
    REDIS_USE_TLS: z.boolean().default(true),
});

export type Config = z.infer<typeof configSchema>;
