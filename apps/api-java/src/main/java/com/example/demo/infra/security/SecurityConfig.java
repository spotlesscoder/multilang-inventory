package com.example.demo.infra.security;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.method.configuration.EnableMethodSecurity;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.AuthenticationFilter;

@Configuration
@EnableMethodSecurity(prePostEnabled = true)
@EnableWebSecurity
public class SecurityConfig {

  private AuthenticationFilter authenticationFilter;

  public SecurityConfig(AuthenticationFilter authenticationFilter) {
    this.authenticationFilter = authenticationFilter;
  }

  @Bean
  public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
    http.csrf(Customizer.withDefaults())
        .authorizeHttpRequests(Customizer.withDefaults())
        .addFilterBefore(
            authenticationFilter,
            org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter
                .class);

    return http.build();
  }
}
