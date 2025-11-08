#!/bin/bash

./scripts/remote_dev/run_command_on_guests.sh "sudo sed -i 's/enable_payment = true/enable_payment = false/g' /etc/lit/node.config.toml"

sleep 10

./scripts/remote_dev/run_command_on_guests.sh "sudo systemctl restart lit-node"