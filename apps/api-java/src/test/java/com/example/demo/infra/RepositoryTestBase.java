package com.example.demo.infra;

import com.mongodb.client.MongoDatabase;
import org.junit.jupiter.api.AfterEach;
import org.springframework.beans.factory.annotation.Autowired;

public class RepositoryTestBase {

  @Autowired private MongoDatabase db;

  @AfterEach
  void cleanUp() {
    db.drop();
  }
}
