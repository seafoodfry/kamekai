# Infra

```sh
. ./setup_env_vars.sh

./run-cmd-in-shell.sh terraform init
./run-cmd-in-shell.sh terraform plan -out a.plan
./run-cmd-in-shell.sh terraform apply "a.plan"
```

To clean up
```sh
./run-cmd-in-shell.sh terraform destroy -auto-approve
```

To test the app
```sh
curl https://api.seafoodfry.ninja/translate -X POST -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" -d '{"text":"meant to be"}' | jq .
```

To copy things:
```sh
#ssh ubuntu@${EC2} -t 'mkdir -p /home/ubuntu/src'

rsync -rvzP --exclude "target" ../backend/ ubuntu@${EC2}:/home/ubuntu/backend

# or...
scp ../backend/publish-in-linux.sh ubuntu@${EC2}:/home/ubuntu/kamekai/backend
```

```sh
ssh ubuntu@${EC2}

tmux new -s kamekai

#...

tmux detach
tmux ls
```

---
## Storage

```
df -h
```

```
df -ih
```

```
sudo du --max-depth=1 -h [--inodes] /
```

```
sudo du --max-depth=1 -h /var/lib/docker | sort -h
```