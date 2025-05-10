package com.example.demo.products;

import lombok.Data;

// FIELD_NAME_NAME

@Data
public class Product {
  public static final String FIELD_NAME_ID = "_id";
  public static final String FIELD_NAME_NAME = "name";
  public static final String FIELD_NAME_CATEGORY = "category";

  private String _id;
  private String category;
  private String name;
}
