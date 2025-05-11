package com.example.demo.products.commands.add;

import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.example.demo.products.ProductsService;
import com.example.demo.products.api.ProductDto;

import jakarta.validation.Valid;

@RestController
@RequestMapping("/products")
public class AddProductController {

  private ProductsService productsService;

  public AddProductController(ProductsService productsService) {
    this.productsService = productsService;
  }

  @PostMapping
  @PreAuthorize("hasAuthority(CREATE_PRODUCT)")
  public ProductDto run(@Valid AddProductRequestDto dto) {
    return ProductDto.fromModel(this.productsService.create(dto.toModel()));
  }
}
