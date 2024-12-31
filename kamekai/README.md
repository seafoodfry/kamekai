# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Debugging

See https://v2.tauri.app/plugin/logging/


## Updating Deps

If there is a newer version of `pnpm`, it'll usually result in a message whenever we use it.
For example:
```
$ pnpm add oidc-client-ts react-oidc-context
   ╭──────────────────────────────────────────────────────────────────╮
   │                                                                  │
   │                Update available! 9.15.1 → 9.15.2.                │
   │   Changelog: https://github.com/pnpm/pnpm/releases/tag/v9.15.2   │
   │         Run "corepack install -g pnpm@9.15.2" to update.         │
   │                                                                  │
   ╰──────────────────────────────────────────────────────────────────╯
```


https://repost.aws/knowledge-center/cognito-custom-domain-errors

```
│ Error: creating Cognito User Pool Domain (auth.seafoodfry.ninja): operation error Cognito Identity Provider: CreateUserPoolDomain, https response error StatusCode: 400, RequestID: XXX, InvalidParameterException: Custom domain is not a valid subdomain: Was not able to resolve a DNS A record for the parent domain or domain parent is a top-level domain.
│ 
│   with aws_cognito_user_pool_domain.kamekai,
│   on cognito.tf line 166, in resource "aws_cognito_user_pool_domain" "kamekai":
│  166: resource "aws_cognito_user_pool_domain" "kamekai" {
```

```
dig A seafoodfry.ninja +short
```