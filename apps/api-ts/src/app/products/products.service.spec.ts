import { Test, TestingModule } from '@nestjs/testing';
import { testLogger } from '../../test/noop-logger';
import { sampleProduct } from '../../test/products/products.testdata';
import { withId } from '../../test/with-id';
import { ProductsRepository } from './db/products.repository';
import { ProductsService } from './products.service';

describe(ProductsService.name, () => {
    let app: TestingModule;
    let repository: ProductsRepository;
    let underTest: ProductsService;

    beforeAll(async () => {
        app = await Test.createTestingModule({
            providers: [
                {
                    provide: ProductsRepository,
                    useValue: {
                        create: jest
                            .fn()
                            .mockImplementation((entity) =>
                                Promise.resolve(withId(entity))
                            ),
                    },
                },
                ProductsService,
            ],
        })
            .setLogger(testLogger)
            .compile();

        repository = app.get<ProductsRepository>(ProductsRepository);
        underTest = app.get<ProductsService>(ProductsService);
    });

    afterEach(() => {
        jest.clearAllMocks();
    });

    it('creates product successfully', async () => {
        // setup
        const repositoryCreate = jest.spyOn(repository, 'create');

        // run
        const product = await underTest.create(sampleProduct());

        // verify
        expect(product).toBeDefined();
        expect(product._id.length).toBeGreaterThanOrEqual(1);
        expect(repositoryCreate).toHaveBeenCalledTimes(1);
    });
});
