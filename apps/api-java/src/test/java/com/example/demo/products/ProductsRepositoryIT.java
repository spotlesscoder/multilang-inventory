package com.example.demo.products;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.example.demo.infra.RepositoryTestBase;
import com.mongodb.assertions.Assertions;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.ActiveProfiles;

@ActiveProfiles("test")
@SpringBootTest
class ProductsRepositoryIT extends RepositoryTestBase {

  private final ProductsRepository repository;

  @Autowired
  public ProductsRepositoryIT(ProductsRepository repository) {
    this.repository = repository;
  }

  @Test
  void saveAndFindByCategory() {
    Product product = new Product();
    product.setName("Banana");
    product.setCategory("electronics");

    repository.save(product);
    List<Product> found = repository.findByCategory("electronics").stream().toList();

    Assertions.assertNotNull(found);
    assertEquals("Banana", found.get(0).getName());
  }
}
