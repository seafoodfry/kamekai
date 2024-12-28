# Infra

```
export TF_VAR_my_ip=$(curl https://cloudflare.com/cdn-cgi/trace | grep ip | awk -F= '{print $2}')

./run-cmd-in-shell.sh terraform init
./run-cmd-in-shell.sh terraform plan -out a.plan
./run-cmd-in-shell.sh terraform apply "a.plan"
```

To clean up
```
./run-cmd-in-shell.sh terraform destroy -auto-approve
```

To test the app
```
ENDPOINT=api.seafoodfry.ninja
curl "${ENDPOINT}" -X POST -H "Content-Type: application/json" -d '{"text":"luck"}'  | jq .
```