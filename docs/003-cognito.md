# Cognito

---
## User Pool

> With the USER_AUTH sign-in flow, users can choose their primary sign-in factor from a list of options
> like password, passwordless, and passkey. Choose the types of authentication that you want to allow
> for users' first authentication prompt.

"Passwordless" may show as unavailable for the following reason:

> You can't configure additional choice options because multi-factor authentication (MFA) is required
> in this user pool. To enable more first authentication factor choices, set MFA to Optional or Off.

```hcl
# Setting this as ON will prevent passwordless login options.
mfa_configuration = "OPTIONAL"
```


## Manual Operations

### Threat Protection

* **Compromised credentials responses** 
* **Automatic risk response** notify user. It requires "Adaptive authentication messages".
    * "Adaptive authentication messages" Enter or select an email address that is verified with Amazon SES in the AWS Region you chose. This email address will be shown as the sender (the FROM email address) on messages sent by Amazon Cognito through Amazon SES.

### Setup MFA

* Enable passwordless options
* **MFA methods** email msgs


---
## App Client


### Oauth2 Flows

> `allowed_oauth_flows` - (Optional) List of allowed OAuth flows, including `code`, `implicit`,
> and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to true before you
> can configure this option.


**Code**

It enables a client application to obtain authorized access to protected resources like web APIs.
The auth code flow requires a user-agent that supports redirection from the authorization server
back to your application.

Sometimes you may see **PKCE** listed as a different "flow" but it is an extension to the OAuth 2.0 Authorization Code flow.
It is designed to enhance its security when used with public clients, like mobile applications or single-page web apps. These clients can’t reliably store client secrets.

Before this enhacement, the code flow was recommended for apps where the client secret can be securely stored by the client itself.
For example, server-side web apps where the source is not publicly exposed.

In the PKCE-enhanced Authorization Code Flow,
the app must create a cryptographically-random `code_verifier` and from this generates a `code_challenge`.
The authorization server stores the `code_challenge` (sent along with a request to the `/authorize` endnpoint)
and redirects the user back to the application with an authorization code, which is good for one use.
The client must then send this code and the `code_verifier` to the authorization server's `/oauth/token`
(or simply `/token`) endpoint.
The authorization server verifies the `code_challenge` and `code_verifier`.
Then the authorization server responds with an ID token and access token, and optionally, a refresh token.

Thus, this flow requires the client to create a code verifier secret.
From the code verifier, a code challenge is generated and sent to the authorization server.
The authorization server replies with an authorization code and stores the code challenge.
The authorization code can then be only exchanged for an access token when it sends back the authorization code along with the code verifier secret that was used to generate the code verifier
secret the authorization server is already holding.
This way, if the authorization code is leaked or stolen, a malicious attacker cannot exchange it
for a token without also stealing the code verifier secret.

Use the auth code flow paired with Proof Key for Code Exchange (PKCE) and OpenID Connect (OIDC) to get access tokens and ID tokens in these types of apps:

- Single-page web application (SPA)
- Standard (server-based) web application
- Desktop and mobile apps


See
- [frontegg.com/blog/oauth-flows](https://frontegg.com/blog/oauth-flows)
- [learn.microsoft.com/identity-platform/v2-oauth2-auth-code-flow](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow)
- [auth0.com/authentication-and-authorization-flow/authorization-code-flow-with-pkce](https://auth0.com/docs/get-started/authentication-and-authorization-flow/authorization-code-flow-with-pkce)
- [rfc6749#section-1.3.1](https://datatracker.ietf.org/doc/html/rfc6749#section-1.3.1)

**Implicit**

This is often also called a "hybrid flow".
It is suitable for applications that can store client secrets, this flow enables immediate access to ID tokens alongside ongoing access to refresh tokens.
Tokens (ID tokens or access tokens) are returned directly from the `/authorize` endpoint instead of the
`/token` endpoint.

Traditionally, there are two ways to return the tokens and this setting is often refered to as the `response_mode`.
When the `response_mode=form_post`
the token is delivered securely through an HTML form POST to the client's redirect URI.
This method ensures that the token isn't exposed in the URL fragment, which in turn avoids the risks
of token leakage through browser history or referrer headers.

The other `response_mode` available for this flow is `fragment`, which means that the tokens are returned in the URL frament or as q query parameter.
The URL fragment is the part of the URL that comes after the `#` symbol and is not sent to the server when the browser requests a new page, but is available to JavaScript running in the browser.
This means that the token is exposed to any JavaScript running on the page, which could be a security risk as most pages include third-party scripts.


See
- [rfc6749#section-1.3.2](https://datatracker.ietf.org/doc/html/rfc6749#section-1.3.2)
- [learn.microsoft.com/identity-platform/v2-oauth2-implicit-grant-flow#security-concerns-with-implicit-grant-flow](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-implicit-grant-flow#security-concerns-with-implicit-grant-flow)


**Client Credentials**

The OAuth 2.0 client credentials grant flow permits a web service (confidential client) to use its own credentials, instead of impersonating a user, to authenticate when calling another web service.
This type is commonly used for server-to-server interactions that must run in the background, without immediate interaction with a user, and is often referred to as daemons or service accounts.

In the client credentials flow, permissions are granted directly to the application itself by an administrator.

This is another way of saying:
that the client, the app, is the **resource owner**.
Also that this flow is for server-to-server communicaiton.
And because the "admin" directly provides permissions for the client, the
returned access token will grant access to specific, predefined scopes.

As a side note, refresh tokens will never be granted with this flow as `client_id` and `client_secret` (which would be required to obtain a refresh token) can be used to obtain an access token instead.

See
[learn.microsoft.com/identity-platform/v2-oauth2-client-creds-grant-flow](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-client-creds-grant-flow)


### Oauth2 Scopes

The OAuth scopes control what user information your app can access:

- `openid`: Basic authentication (required for OIDC)
- `email`: Access to user's email address
- `profile`: Access to user profile info like name, picture, locale

Each scope returns specific claims in the ID token:
```json
{
    "openid": ["sub"],
    "email": ["email", "email_verified"],
    "profile": ["name", "family_name", "given_name", "middle_name", "nickname", "preferred_username", "picture", "locale"]
}
```

Access tokens are defined in OAuth, ID tokens are defined in OpenID Connect.
An ID token contains information about what happened when a user authenticated, and is intended to be read by the OAuth client (our app in our case - that's how we'd get info about the user).

See
[oauth.net/id-tokens-vs-access-tokens/](https://oauth.net/id-tokens-vs-access-tokens/).


---
## Implementation

```hcl
# Client Secret is required to set EnablePropagateAdditionalUserContextData as true
enable_propagate_additional_user_context_data = true
```

```
pnpm add oidc-client-ts react-oidc-context
```

```js
// index.js
import { AuthProvider } from "react-oidc-context";

const cognitoAuthConfig = {
  authority: "https://cognito-idp.us-east-1.amazonaws.com/us-east-XXX",
  client_id: "CLIENTID",
  redirect_uri: "http://localhost:8080/auth/callback",
  response_type: "code",
  scope: "email openid profile",
};

const root = ReactDOM.createRoot(document.getElementById("root"));

// wrap the application with AuthProvider
root.render(
  <React.StrictMode>
    <AuthProvider {...cognitoAuthConfig}>
      <App />
    </AuthProvider>
  </React.StrictMode>
);
```

```js
// App.js

import { useAuth } from "react-oidc-context";

function App() {
  const auth = useAuth();

  const signOutRedirect = () => {
    const clientId = "6k8a9akeqj6s5usmj09aendjfd";
    const logoutUri = "http://localhost:8080/auth/logout";
    const cognitoDomain = "https://<user pool domain>";
    window.location.href = `${cognitoDomain}/logout?client_id=${clientId}&logout_uri=${encodeURIComponent(logoutUri)}`;
  };

  if (auth.isLoading) {
    return <div>Loading...</div>;
  }

  if (auth.error) {
    return <div>Encountering error... {auth.error.message}</div>;
  }

  if (auth.isAuthenticated) {
    return (
      <div>
        <pre> Hello: {auth.user?.profile.email} </pre>
        <pre> ID Token: {auth.user?.id_token} </pre>
        <pre> Access Token: {auth.user?.access_token} </pre>
        <pre> Refresh Token: {auth.user?.refresh_token} </pre>

        <button onClick={() => auth.removeUser()}>Sign out</button>
      </div>
    );
  }

  return (
    <div>
      <button onClick={() => auth.signinRedirect()}>Sign in</button>
      <button onClick={() => signOutRedirect()}>Sign out</button>
    </div>
  );
}
  
export default App;
```


---
# Backend


```sh
curl https://cognito-idp.us-east-1.amazonaws.com/us-east-1_NipWF0bjs/.well-known/jwks.json | jq .
```
```json
{
  "keys": [
    {
      "alg": "RS256",
      "e": "AQAB",
      "kid": "XfFX3LUezTscwo4DnRLxwMg6ND7Y4E3zDI2myYLoidE=",
      "kty": "RSA",
      "n": "...",
      "use": "sig"
    },
    {
      "alg": "RS256",
      "e": "AQAB",
      "kid": "+DccI9o4Kr8AyOYKZ2QYYnb2AACO49lK1s7wb/sggeY=",
      "kty": "RSA",
      "n": "...",
      "use": "sig"
    }
  ]
}
```

`alg`, defined in 4.4 in RFC 7517.

> The "alg" (algorithm) parameter identifies the algorithm intended for use with the key.
> The values used should either be registered in the IANA "JSON Web Signature and Encryption Algorithms" registry
> established by [JWA - RFC 7518] or be a value that contains a Collision-Resistant Name.

`e`, defined in 6.3.1.2 in RFC 7518.

> The "e" (exponent) parameter contains the exponent value for the RSA public key.
> It is represented as a Base64urlUInt-encoded value.
> For instance, when representing the value 65537, the octet sequence to be base64url-encoded MUST consist
> of the three octets [1, 0, 1]; the resulting representation for this value is "AQAB".

`kty`, defined in 6.1 in RFC 7518.
Stands for Key TYpe.

`n`, defined in 6.3.1.1 in RFC 7518.

> The "n" (modulus) parameter contains the modulus value for the RSA public key.
> It is represented as a Base64urlUInt-encoded value.
> Note that implementers have found that some cryptographic libraries prefix an extra zero-valued octet to
> the modulus representations they return, for instance, returning 257 octets for a 2048-bit key,
> rather than 256.  Implementations using such libraries will need to take care to omit the extra octet from
> the base64url-encoded representation.

`use`, deinfed in 4.2. in RFC 7517.

> "sig" (signature) or "enc" (encryption).


JSON is Base64Url encoded to form the first part of the JWT.

Header
```json
{ "kid": "XfFX3LUezTscwo4DnRLxwMg6ND7Y4E3zDI2myYLoidE=", "alg": "RS256" }
```

The `kid` is the key used to sign the JWT.

JWTs can be signed using a secret (with the HMAC algorithm) or a public/private key pair using RSA or ECDSA.
RS256 (RSA Signature with SHA-256) is an asymmetric algorithm that uses a public/private key pair. The identity provider has a private key to generate the signature.


Payload:
```json
{ 
  "at_hash": "xxxxxxxxx",
  "sub": "USERID",
  "email_verified": true,
  "iss": "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_NipWF0bjs",
  "cognito:username": "SAME AS USERID (SUB)",
  "origin_jti": "token-revocation identifier",
  "aud": "COGNITO_APP_ID",
  "token_use": "id",
  "auth_time": 1735924608,
  "exp": 1735928208,
  "iat": 1735924608,
  "jti": "JWT ID TOKEN ID",
  "email": "REDACTED"
}
```

`at_hash`, defined in 3.1.3.6 ID Token on OIDC Core.
This field is require on ID Token Claims when using the Authorization Code Flow:

> OPTIONAL. Access Token hash value. Its value is the base64url encoding of the left-most half of the hash of the
> octets of the ASCII representation of the access_token value, where the hash algorithm used is the hash
> algorithm used in the alg Header Parameter of the ID Token's JOSE Header.
> For instance, if the alg is RS256, hash the access_token value with SHA-256,
> then take the left-most 128 bits and base64url-encode them. The at_hash value is a case-sensitive string.


`sub`, defined in 4.1.2 in RFC 7519.
This is the user ID in cognito.

> The "sub" (subject) claim identifies the principal that is the subject of the JWT.
> The claims in a JWT are normally statements about the subject. The subject value MUST either be scoped
> to be locally unique in the context of the issuer or be globally unique. The processing of this claim is
> generally application specific. The "sub" value is a case-sensitive string containing a StringOrURI
> value. Use of this claim is OPTIONAL.
>
> Subject - Identifier for the End-User at the Issuer.

`email_verified` is listed in 5.1 Standard Claims in OIDC Core.

`iss`, defined in 4.1.1 in RFC 7519.

> The "iss" (issuer) claim identifies the principal that issued the JWT.
> The processing of this claim is generally application specific.

`origin_jti`, an AWS cognito thing.

> A token-revocation identifier associated with your user's refresh token. Amazon Cognito references the
> origin_jti claim when it checks if you revoked your user's token with the Revoke endpoint or the RevokeToken API
> operation. When you revoke a token, Amazon Cognito invalidates all access and ID tokens with the same
> origin_jti value.


`aud`, defined in 4.1.3 in RFC 7519.
The user pool app client that authenticated your user.
Amazon Cognito renders the same value in the access token client_id claim.

> The "aud" (audience) claim identifies the recipients that the JWT is intended for.
> Each principal intended to process the JWT MUST identify itself with a value in the audience claim.
> If the principal processing the claim does not identify itself with a value in the "aud" claim when
> this claim is present, then the JWT MUST be rejected.
>
> The Authorization Server MUST verify that it is an intended audience for the token.
> The Audience SHOULD be the URL of the Authorization Server's Token Endpoint. [OIDC CORE]

`jti`, defined in OIDC Core section 9.

> JWT ID. A unique identifier for the token, which can be used to prevent reuse of the token.
> These tokens MUST only be used once, unless conditions for reuse were negotiated between the parties;
> any such negotiation is beyond the scope of this specification.

## References

1. [RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519) - JSON Web Token (JWT)
1. [RFC 7518](https://datatracker.ietf.org/doc/html/rfc7518) - JSON Web Algorithms (JWA)
1. [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517) - JSON Web Key (JWK)
1. [RFC 7515](https://datatracker.ietf.org/doc/html/rfc7515) - JSON Web Signature (JWS)
1. [RFC 9068](https://datatracker.ietf.org/doc/html/rfc9068) - JSON Web Token (JWT) Profile for OAuth 2.0 Access Tokens
1. [OpenID Connect Core](https://openid.net/specs/openid-connect-core-1_0.html) – Defines the core OpenID Connect functionality: authentication built on top of OAuth 2.0 and the use of claims to communicate information about the End-User.
1. [RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749) The OAuth 2.0 Authorization Framework
1. [AWS Understanding the identity (ID) token](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-id-token.html)

And for additional reading:
[REF 3447: Public-Key Cryptography Standards (PKCS) #1: RSA Cryptography Specifications Version 2.1](https://datatracker.ietf.org/doc/html/rfc3447).