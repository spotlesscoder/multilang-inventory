import { Injectable, Logger } from '@nestjs/common';
import dotenv from 'dotenv';
import { Config, configSchema } from './config.schema';

@Injectable()
export class ConfigService {
    private readonly logger = new Logger(ConfigService.name);

    private readonly config: Config;

    constructor() {
        const env = dotenv.config().parsed ?? {};
        const parsed = configSchema.safeParse(env);

        if (!parsed.success) {
            this.logger.error(
                '❌ Invalid environment variables:',
                parsed.error.format()
            );
            throw new Error('Invalid environment variables');
        }

        this.config = parsed.data;
    }

    get<K extends keyof Config>(key: K): Config[K] {
        return this.config[key];
    }
}
