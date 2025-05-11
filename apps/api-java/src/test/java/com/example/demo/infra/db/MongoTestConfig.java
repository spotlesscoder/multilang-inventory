package com.example.demo.infra.db;

import static com.aventrix.jnanoid.jnanoid.NanoIdUtils.randomNanoId;

import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoDatabase;
import jakarta.annotation.PostConstruct;
import org.bson.Document;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Primary;
import org.springframework.context.annotation.Profile;

@Profile("test")
@Configuration
public class MongoTestConfig {

  @Value("${db.URI}")
  private String dbURI;

  private MongoClient mongoClient;
  private MongoDatabase adminDb;

  @Bean
  public MongoClient mongoClient() {
    System.out.println("MongoClient created");
    this.mongoClient = MongoClients.create(dbURI);
    return this.mongoClient;
  }

  @Bean
  @Primary
  public MongoDatabase mongoDatabase(MongoClient mongoClient) {
    String dbName = "test-db-" + randomNanoId();
    return mongoClient.getDatabase(dbName);
  }

  @PostConstruct
  public void enableNotablescan() {
    if (mongoClient == null) {
      mongoClient = MongoClients.create(dbURI);
    }
    adminDb = mongoClient.getDatabase("admin");
    adminDb.runCommand(new Document("setParameter", 1).append("notablescan", true));
    System.out.println("✅ Enabled notablescan");
  }
}
