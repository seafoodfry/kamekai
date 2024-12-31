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