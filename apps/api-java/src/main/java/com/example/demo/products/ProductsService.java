package com.example.demo.products;

import org.springframework.stereotype.Service;

@Service
public class ProductsService {

  private final ProductsRepository repository;

  public ProductsService(final ProductsRepository repository) {
    this.repository = repository;
  }

  public void create(Product product) {
    repository.save(product);
  }
}
