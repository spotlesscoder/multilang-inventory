package com.example.demo.infra.auth;

import jakarta.servlet.http.HttpServletRequest;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.AuthenticationException;

public interface AuthnProvider {
  boolean supports(HttpServletRequest request);

  Authentication authenticate(HttpServletRequest request) throws AuthenticationException;
}
