package com.example.demo.products.db;

import static com.example.demo.infra.db.Id.id;
import static com.example.demo.products.db.Product.FIELD_NAME_CATEGORY;
import static com.example.demo.products.db.Product.FIELD_NAME_NAME;

import com.mongodb.client.FindIterable;
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import java.util.Collection;
import java.util.List;
import java.util.Optional;
import org.bson.Document;
import org.springframework.stereotype.Repository;

@Repository
public class ProductsRepository {

  private final MongoCollection<Document> collection;

  public ProductsRepository(MongoDatabase database) {
    this.collection = database.getCollection("products");
    collection.createIndex(new Document(Product.FIELD_NAME_CATEGORY, 1));
  }

  public Product save(Product product) {
    Document doc =
        new Document(Product.FIELD_NAME_NAME, product.getName())
            .append(Product.FIELD_NAME_CATEGORY, product.getCategory())
            .append(Product.FIELD_NAME_ID, id());

    product.set_id(collection.insertOne(doc).getInsertedId().toString());
    return product;
  }

  public Optional<Product> findByName(String name) {
    Document doc = collection.find(new Document(Product.FIELD_NAME_NAME, name)).first();
    if (doc != null) {
      Product product = new Product();
      product.set_id(id());
      product.setName(doc.getString(FIELD_NAME_NAME));
      product.setCategory(doc.getString(FIELD_NAME_CATEGORY));
      return Optional.of(product);
    }

    return Optional.empty();
  }

  public Collection<Product> findByCategory(String string) {
    FindIterable<Document> docs = collection.find(new Document("category", string));
    List<Product> products = new java.util.ArrayList<>();
    for (Document doc : docs) {
      Product product = new Product();
      product.set_id(doc.getString("_id"));
      product.setName(doc.getString(FIELD_NAME_NAME));
      product.setCategory(doc.getString(FIELD_NAME_CATEGORY));
      products.add(product);
    }
    return products;
  }
}
