package com.example.demo.products.api;

import com.example.demo.products.db.Product;
import lombok.Data;

@Data
public class ProductDto {

  private String id;

  private String category;

  private String name;

  public static ProductDto fromModel(Product model) {
    ProductDto result = new ProductDto();
    result.setId(model.get_id());
    result.setCategory(model.getCategory());
    result.setName(model.getName());

    return result;
  }
}
