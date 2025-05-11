package com.example.demo.infra.auth;

import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.List;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.filter.OncePerRequestFilter;

public class AuthnFilter extends OncePerRequestFilter {

  private final List<AuthnProvider> providers;

  public AuthnFilter(List<AuthnProvider> providers) {
    this.providers = providers;
  }

  @Override
  protected void doFilterInternal(
      HttpServletRequest request, HttpServletResponse response, FilterChain chain)
      throws ServletException, IOException {

    for (AuthnProvider provider : providers) {
      if (provider.supports(request)) {
        try {
          var authentication = provider.authenticate(request);
          SecurityContextHolder.getContext().setAuthentication(authentication);
        } catch (Exception ex) {
          response.sendError(HttpServletResponse.SC_UNAUTHORIZED, "Authentication Failed");
          return;
        }
        break;
      }
    }

    chain.doFilter(request, response);
  }
}
