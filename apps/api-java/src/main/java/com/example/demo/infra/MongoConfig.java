package com.example.demo.infra;

import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoDatabase;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Profile;

@Profile("!test")
@Configuration
public class MongoConfig {

  @Value("${mongodb.uri}")
  private String mongoURI;

  @Value("${mongodb.database}")
  private String dbName;

  @Bean
  public MongoClient mongoClient() {
    return MongoClients.create(mongoURI);
  }

  @Bean
  public MongoDatabase mongoDatabase(MongoClient mongoClient) {
    return mongoClient.getDatabase(dbName);
  }
}
