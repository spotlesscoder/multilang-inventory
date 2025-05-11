package com.example.demo.infra.db;

import static com.aventrix.jnanoid.jnanoid.NanoIdUtils.randomNanoId;

public class Id {

  private Id() {
    // Prevent instantiation
  }

  public static String id() {
    return randomNanoId();
  }
}
