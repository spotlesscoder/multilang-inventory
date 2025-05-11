import { CacheModule } from '@nestjs/cache-manager';
import { Module } from '@nestjs/common';
import { EventEmitterModule } from '@nestjs/event-emitter';
import { MongooseModule } from '@nestjs/mongoose';
import { redisStore } from 'cache-manager-ioredis-yet';
import { ConfigModule } from './config/config.module';
import { ConfigService } from './config/config.service';

@Module({
    imports: [
        CacheModule.registerAsync({
            imports: [ConfigModule],
            inject: [ConfigService],
            useFactory: async (configService: ConfigService) => {
                const store = await redisStore({
                    host: configService.get('REDIS_HOST'),
                    port: configService.get('REDIS_PORT'),
                    password: configService.get('REDIS_PASSWORD'),
                });

                return {
                    store: store,
                };
            },
            isGlobal: true,
        }),
        ConfigModule,
        MongooseModule.forRoot('mongodb://localhost:27017'),
        EventEmitterModule.forRoot(),
    ],
})
export class InfrastructureModule {}
