package com.example.demo.products.api;

import com.example.demo.products.ProductsService;
import jakarta.validation.Valid;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/products")
public class ProductsController {

  private ProductsService productsService;

  public ProductsController(ProductsService productsService) {
    this.productsService = productsService;
  }

  @PostMapping
  @PreAuthorize("hasAuthority(CREATE_PRODUCT)")
  public ProductDto createProduct(@Valid CreateProductDto dto) {
    return ProductDto.fromModel(this.productsService.create(dto.toModel()));
  }
}
