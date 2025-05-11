package com.example.demo.infra.auth.apikey.db;

import java.time.Instant;
import java.util.Set;
import lombok.Data;

@Data
public class ApiKey {

  public static final String FIELD_NAME_ID = "_id";
  public static final String FIELD_NAME_KEY = "key";
  public static final String FIELD_NAME_PERMISSIONS = "permissions";
  public static final String FIELD_NAME_CREATED_AT = "createdAt";
  public static final String FIELD_NAME_EXPIRES_AT = "expiresAt";

  private String _id;
  private String key;
  private Set<String> permissions;
  private Instant createdAt = Instant.now();
  private Instant expiresAt;
}
