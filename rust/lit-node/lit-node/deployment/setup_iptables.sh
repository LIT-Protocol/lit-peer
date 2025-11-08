#!/bin/bash

NODE_IP=<Your node IP>

sudo iptables -t nat -A PREROUTING -i eno1 -p tcp --dport 7470 -j REDIRECT --to-port 7370
sudo iptables -t nat -A PREROUTING -i eno1 -p tcp --dport 7471 -j REDIRECT --to-port 7371
sudo iptables -t nat -A PREROUTING -i eno1 -p tcp --dport 7472 -j REDIRECT --to-port 7372
sudo iptables -t nat -A OUTPUT -p tcp --dport 7470 -d $NODE_IP -j DNAT --to-destination 127.0.0.1:7470
sudo iptables -t nat -A OUTPUT -p tcp --dport 7471 -d $NODE_IP -j DNAT --to-destination 127.0.0.1:7471
sudo iptables -t nat -A OUTPUT -p tcp --dport 7472 -d $NODE_IP -j DNAT --to-destination 127.0.0.1:7472
sudo iptables -t nat -A OUTPUT -p tcp --dport 27078 -d $NODE_IP -j DNAT --to-destination 127.0.0.1:27078
sudo iptables -t nat -A OUTPUT -p tcp --dport 27079 -d $NODE_IP -j DNAT --to-destination 127.0.0.1:27079
sudo iptables -t nat -A OUTPUT -p tcp --dport 27080 -d $NODE_IP -j DNAT --to-destination 127.0.0.1:27080
