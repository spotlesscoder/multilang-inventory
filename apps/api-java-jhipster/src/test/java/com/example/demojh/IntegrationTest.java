package com.example.demojh;

import com.example.demojh.config.AsyncSyncConfiguration;
import com.example.demojh.config.EmbeddedElasticsearch;
import com.example.demojh.config.EmbeddedKafka;
import com.example.demojh.config.EmbeddedMongo;
import com.example.demojh.config.JacksonConfiguration;
import com.example.demojh.config.TestSecurityConfiguration;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import org.springframework.boot.test.context.SpringBootTest;

/**
 * Base composite annotation for integration tests.
 */
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.RUNTIME)
@SpringBootTest(classes = { JhdemoApp.class, JacksonConfiguration.class, AsyncSyncConfiguration.class, TestSecurityConfiguration.class })
@EmbeddedMongo
@EmbeddedElasticsearch
@EmbeddedKafka
public @interface IntegrationTest {
}
