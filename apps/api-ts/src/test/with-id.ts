import { id } from '../app/infra/id';

export function withId<T>(entity: T): T & { _id: string } {
    return {
        ...entity,
        _id: id(),
    };
}
