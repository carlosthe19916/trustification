resource "aws_cognito_user_pool" "pool" {
  name = "trustification-${var.environment}"
}

variable "sso-domain" {
  type = string
}

variable "admin-email" {
  type = string
}

resource "aws_cognito_user_pool_domain" "main" {
  domain       = var.sso-domain
  user_pool_id = aws_cognito_user_pool.pool.id
}

resource "aws_cognito_user" "admin" {
  user_pool_id = aws_cognito_user_pool.pool.id
  username     = "admin"

  attributes = {
    email          = var.admin-email
    email_verified = true
  }
}

resource "aws_cognito_user_pool_client" "walker" {
  name         = "frontend-${var.environment}"
  user_pool_id = aws_cognito_user_pool.pool.id

  supported_identity_providers = ["COGNITO"]

  #allowed_oauth_flows_user_pool_client = true
  allowed_oauth_flows                  = ["client_credentials"]
  allowed_oauth_scopes                 = []
  generate_secret                      = true
}

resource "kubernetes_secret" "oidc-walker" {
  metadata {
    name      = "oidc-walker"
    namespace = var.namespace
  }

  data = {
    client-id     = aws_cognito_user_pool_client.walker.id
    client-secret = aws_cognito_user_pool_client.walker.client_secret
  }

  type = "Opaque"
}

resource "aws_cognito_user_pool_client" "frontend" {
  name         = "walker-${var.environment}"
  user_pool_id = aws_cognito_user_pool.pool.id

  supported_identity_providers = ["COGNITO"]

  allowed_oauth_flows_user_pool_client = true
  allowed_oauth_flows                  = ["code"]
  allowed_oauth_scopes                 = ["email", "openid"]
  callback_urls                        = ["https://console-trustification-jreimann.apps.cluster.trustification.rocks"]
}

resource "kubernetes_secret" "oidc-frontend" {
  metadata {
    name      = "oidc-frontend"
    namespace = var.namespace
  }

  data = {
    client-id = aws_cognito_user_pool_client.frontend.id
  }

  type = "Opaque"
}