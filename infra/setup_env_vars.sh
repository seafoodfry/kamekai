#!/bin/sh

export TF_VAR_cloudflare_api_token=$(op read "op://eng-vault/cloudflare-api-token/credential")