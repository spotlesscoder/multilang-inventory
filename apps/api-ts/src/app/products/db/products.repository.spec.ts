import { getModelToken } from '@nestjs/mongoose';
import { Test, TestingModule } from '@nestjs/testing';
import { Model } from 'mongoose';
import { createDb } from '../../../test/db-test.module';
import { testLogger } from '../../../test/noop-logger';
import { sampleProduct } from '../../../test/products/products.testdata';
import { ProductsRepository } from './products.repository';
import { Product, ProductDoc, ProductModel } from './schemas/product.schema';

describe(ProductsRepository.name, () => {
    let app: TestingModule;
    let model: Model<ProductDoc>;
    let underTest: ProductsRepository;

    beforeAll(async () => {
        app = await Test.createTestingModule({
            imports: await createDb([ProductModel]),
            providers: [ProductsRepository],
        })
            .setLogger(testLogger)
            .compile();

        model = app.get<Model<ProductDoc>>(getModelToken(Product.name));
        underTest = app.get<ProductsRepository>(ProductsRepository);
    });

    beforeEach(async () => {
        await model.deleteMany();
        await model.ensureIndexes();
    });

    it('creates and reads product successfully', async () => {
        // setup
        const doc = sampleProduct();

        // run
        const created = await underTest.create(doc);

        // verify
        const read = await model.findById(created._id).lean();
        expect(read).toMatchObject(doc);
    });
});
