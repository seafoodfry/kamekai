# Infra

```
./run-cmd-in-shell.sh terraform init
./run-cmd-in-shell.sh terraform plan -out a.plan
./run-cmd-in-shell.sh terraform apply "a.plan"
```

To clean up
```
./run-cmd-in-shell.sh terraform destroy
```

To test the app
```
ENDPOINT=https://5jf4mp9ssg.us-east-1.awsapprunner.com/translate
curl "${ENDPOINT}" -X POST -H "Content-Type: application/json" -d '{"text":"luck"}'  | jq .
```