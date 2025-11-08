# Lit Node Operator (`lit-node-operator`)

The `lit-node-operator` is a daemon service that manages and monitors Lit Protocol node instances. It provides essential operational capabilities including Yellowstone Chronicle replica management, blockchain event monitoring, and automated replica node lifecycle management.

## Key Functions

* **Host Command Handling:** Listens for and executes on-chain commands like node restarts or upgrades.
* **Yellowstone Chronicle Replica Management:** Maintains a healthy, synced local RPC endpoint.

## Functionality Details

### Host Command Handling

* Listens to the `HostCommands` smart contract on the configured parent chain.
* Processes `Restart` and `Upgrade` events targeted specifically at this node instance.
* Uses a resilient event monitoring system for reliable command execution.

### Chronicle Replica Management

* **Health Monitoring:** Monitors the `yellowstone` Docker container's status:
  * `Starting`: Container is initializing or syncing initial blocks
  * `Healthy`: Container is fully synced and operational
  * `Unhealthy`: Container is running but out of sync or failing health checks
  * `NotFound`: Container is not running
  * `NoResponse`: Container is unreachable or Docker health commands are failing

* **Access Control:** Uses `iptables` to control traffic to the replica:
  * `ACCEPT`: Allows traffic when the replica is `Healthy`
  * `REJECT`: Blocks traffic in all other states to prevent serving stale data

* **Auto-Recovery:** Implements a robust state machine that:
  * Recreates container if stuck in `Starting` state for too long (15 minutes by default)
  * Recreates container if `Unhealthy` for extended period (2 hours by default)
  * Handles recovery from various failure scenarios

## Prerequisites

* **Root Privileges:** Daemon must run as root to manage Docker and iptables.
* **Docker:** Docker engine installed and running.
* **`iptables`:** Required for firewall management.
* **Scripts & Config:** Required files deployed (typically via Salt):
    * `/var/chronicle/start_yellowstone_replica.sh` (executable)
    * `/var/chronicle/check_replica_sync.sh` (executable)
    * `/var/chronicle/yellowstone/archive.torrent`
* **Network:** Host requires necessary outbound access (RPCs, torrent peers).

## Running the Operator

Typically run as a `systemd` service and setup and installed when the node is setup/upgraded.

### Service Management

```bash
# Start the service
sudo systemctl start lit-node-operator.service

# Stop the service
sudo systemctl stop lit-node-operator.service

# Restart the service
sudo systemctl restart lit-node-operator.service

# Check status
sudo systemctl status lit-node-operator.service
```

### Monitoring Logs

```bash
# View logs in real-time
sudo journalctl -u lit-node-operator.service -f

# View recent logs with timestamps
sudo journalctl -u lit-node-operator.service -n 100 --no-pager

# View logs since a specific time
sudo journalctl -u lit-node-operator.service --since "2023-01-01 00:00:00"
```

### Troubleshooting

If the Chronicle replica is having issues:

1. Check the operator logs for errors:
   ```bash
   sudo journalctl -u lit-node-operator.service | grep -i error
   ```

2. Verify Docker container status:
   ```bash
   docker ps -a | grep yellowstone
   docker inspect --format='{{.State.Health.Status}}' yellowstone
   ```

3. Check iptables rules:
   ```bash
   sudo iptables -L YELLOWSTONE_ACCESS -v
   ```

4. Manually run the recreation script if needed:
   ```bash
   sudo /var/chronicle/start_yellowstone_replica.sh
   ```
