#!/bin/sh

export TF_VAR_my_ip=$(curl https://cloudflare.com/cdn-cgi/trace | grep ip | awk -F= '{print $2}')
export TF_VAR_cloudflare_api_token=$(op read "op://eng-vault/cloudflare-api-token/credential")