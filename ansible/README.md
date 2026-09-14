# Things not handled by ansible:

* Tailscale nameserver override for Split DNS in tailscale admin console > DNS tab (under "Nameservers", "Split DNS" should be set to {{ public_domain_name }} or {{ local_domain_name }} or {{ caddy_domain }}). This should be added to the playbooks.
* Secrets kept in bitwarden
* Creating VMs (or hosts) with debian, networking, a user with sudo and ssh
* DHCP static IPs for hosts and truenas

# How to deploy with ansible

1. `cd ansible` (from repo root)
2. `source $REPOROOT/venv/bin/active`
3. ` export BWS_ACCESS_TOKEN=blah`, note the preceding space to prevent shell history appending
4. `ansible-playbook -i environments/<inventory>/inventory.yml playbooks/<playbook> [--tags <tags>]
