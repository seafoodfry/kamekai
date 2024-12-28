# Infra

```
. ./setup_env_vars.sh

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
curl https://api.seafoodfry.ninja/translate -X POST -H "Content-Type: application/json" \
    -d '{"text":"meant to be"}' | jq .
```