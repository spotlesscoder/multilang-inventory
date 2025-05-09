import { nanoid } from 'nanoid';

export const DEFAULT_ID_LENGTH = 21;

export function id(): string {
    return nanoid();
}
