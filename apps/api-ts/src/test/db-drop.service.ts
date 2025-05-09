import { Injectable, OnModuleDestroy, OnModuleInit } from '@nestjs/common';
import { InjectConnection } from '@nestjs/mongoose';
import { Connection } from 'mongoose';

@Injectable()
export class DbDropService implements OnModuleDestroy, OnModuleInit {
    constructor(@InjectConnection() private connection: Connection) {}

    async onModuleInit() {
        await this.connection.db?.admin().command({
            setParameter: 1,
            notablescan: 1,
        });
    }

    async onModuleDestroy() {
        await this.connection.db?.dropDatabase();
    }
}
