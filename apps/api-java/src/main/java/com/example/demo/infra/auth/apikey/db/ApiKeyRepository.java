package com.example.demo.infra.auth.apikey.db;

import static com.example.demo.infra.db.Id.id;

import com.mongodb.client.FindIterable;
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import org.bson.Document;
import org.springframework.stereotype.Repository;

@Repository
public class ApiKeyRepository {

  private final MongoCollection<Document> collection;

  public ApiKeyRepository(MongoDatabase database) {
    this.collection = database.getCollection("api_keys");
  }

  public void save(ApiKey apiKey) {
    Document doc =
        new Document(ApiKey.FIELD_NAME_CREATED_AT, apiKey.getCreatedAt())
            .append(ApiKey.FIELD_NAME_EXPIRES_AT, apiKey.getExpiresAt())
            .append(ApiKey.FIELD_NAME_KEY, apiKey.getKey())
            .append(ApiKey.FIELD_NAME_PERMISSIONS, apiKey.getPermissions())
            .append(ApiKey.FIELD_NAME_ID, id());
    collection.insertOne(doc);
  }

  public Collection<ApiKey> getAll() {
    FindIterable<Document> docs = collection.find();
    List<ApiKey> apiKeys = new java.util.ArrayList<>();
    for (Document doc : docs) {
      ApiKey apiKey = new ApiKey();
      apiKey.set_id(doc.getString(ApiKey.FIELD_NAME_ID));
      apiKey.setCreatedAt(doc.getDate(ApiKey.FIELD_NAME_CREATED_AT).toInstant());
      apiKey.setExpiresAt(doc.getDate(ApiKey.FIELD_NAME_EXPIRES_AT).toInstant());
      apiKey.setPermissions(
          new HashSet<>(doc.getList(ApiKey.FIELD_NAME_PERMISSIONS, String.class)));
      apiKey.setKey(doc.getString(ApiKey.FIELD_NAME_KEY));
      apiKeys.add(apiKey);
    }
    return apiKeys;
  }
}
