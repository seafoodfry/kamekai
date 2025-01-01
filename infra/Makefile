.PHONY: all
all:
	./run-cmd-in-shell.sh terraform init
	./run-cmd-in-shell.sh terraform plan -out a.plan
	./run-cmd-in-shell.sh terraform apply "a.plan"


.PHONY: clean
clean:
	./run-cmd-in-shell.sh terraform destroy -auto-approve
