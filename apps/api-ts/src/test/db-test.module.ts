import { DynamicModule, Module, Type } from '@nestjs/common';
import { ModelDefinition, MongooseModule } from '@nestjs/mongoose';
import { id } from '../app/infra/id';
import { DbDropService } from './db-drop.service';

export async function createDb(
    models?: ModelDefinition[]
): Promise<(DynamicModule | Type)[]> {
    return [
        DbDropModule,
        MongooseModule.forRootAsync({
            useFactory: () => ({
                autoIndex: true,
                dbName: id(),
                retryAttempts: 10,
                serverSelectionTimeoutMS: 1000,
                uri: 'mongodb://db:27017',
            }),
        }),
        MongooseModule.forFeature(models),
    ];
}
@Module({
    providers: [DbDropService],
})
export class DbDropModule {}
