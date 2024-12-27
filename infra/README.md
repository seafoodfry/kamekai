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