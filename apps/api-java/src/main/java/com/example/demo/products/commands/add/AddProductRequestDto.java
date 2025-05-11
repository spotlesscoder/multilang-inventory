package com.example.demo.products.commands.add;

import com.example.demo.products.db.Product;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import lombok.Data;

@Data
public class AddProductRequestDto {

  @NotBlank @Size(min = 1, max = 255) private String category;

  @NotBlank private String name;

  public Product toModel() {
    Product model = new Product();
    model.setCategory(category);
    model.setName(name);
    return model;
  }
}
