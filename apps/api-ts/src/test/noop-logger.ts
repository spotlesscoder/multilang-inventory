import { LoggerService } from '@nestjs/common';

export class NoopLogger implements LoggerService {
    log() {
        // noop
    }
    error() {
        // noop
    }
    warn() {
        // noop
    }
}

export const testLogger = new NoopLogger();
