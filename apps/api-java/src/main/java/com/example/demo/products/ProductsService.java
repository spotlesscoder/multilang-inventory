package com.example.demo.products;

import com.example.demo.products.db.Product;
import com.example.demo.products.db.ProductsRepository;
import org.springframework.stereotype.Service;

@Service
public class ProductsService {

  private final ProductsRepository repository;

  public ProductsService(final ProductsRepository repository) {
    this.repository = repository;
  }

  public Product create(Product product) {
    return repository.save(product);
  }
}
