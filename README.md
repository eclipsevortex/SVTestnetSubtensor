<div align="center">

# **SubVortex Testnet Subtensor** <!-- omit in toc -->

[![Discord Chat](https://img.shields.io/discord/308323056592486420.svg)](https://discord.gg/bittensor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Discord](https://discord.gg/bittensor) • [Network](https://taostats.io/) • [Research](https://bittensor.com/whitepaper)

</div>

---

- [Installation](#installation)
- [Uninstallation](#uninstallation)

---

# Installation

To install the local subtensor, run 
```bash
pm2 start ./subtensor_start.sh -f \
    --name subtensor -- \
    -e binary \
    --network testnet \
    --node-type lite
```

Once started, you can check the local subtensor is running by looking into the log

```bash
pm2 log subtensor
```

You shoud have something similar to

```bash
0|subtenso | 2024-03-19 23:31:41 🏷  Local node identity is: 12D3KooWPycx2kKpkkwbzjFSAKdTVFRvntLUKkC7VB3P7aUThmfX
0|subtenso | 2024-03-19 23:31:41 💻 Operating system: linux
0|subtenso | 2024-03-19 23:31:41 💻 CPU architecture: x86_64
0|subtenso | 2024-03-19 23:31:41 💻 Target environment: gnu
0|subtenso | 2024-03-19 23:31:41 💻 CPU: AMD EPYC 7282 16-Core Processor
0|subtenso | 2024-03-19 23:31:41 💻 CPU cores: 6
0|subtenso | 2024-03-19 23:31:41 💻 Memory: 16002MB
0|subtenso | 2024-03-19 23:31:41 💻 Kernel: 5.15.0-25-generic
0|subtenso | 2024-03-19 23:31:41 💻 Linux distribution: Ubuntu 22.04.4 LTS
0|subtenso | 2024-03-19 23:31:41 💻 Virtual machine: yes
0|subtenso | 2024-03-19 23:31:41 📦 Highest known block at #0
0|subtenso | 2024-03-19 23:31:41 〽️ Prometheus exporter started at 0.0.0.0:9615
0|subtenso | 2024-03-19 23:31:41 Running JSON-RPC HTTP server: addr=0.0.0.0:9933, allowed origins=["*"]
0|subtenso | 2024-03-19 23:31:41 Running JSON-RPC WS server: addr=0.0.0.0:9944, allowed origins=["*"]
0|subtenso | 2024-03-19 23:31:42 🔍 Discovered new external address for our node: /ip4/155.133.26.129/tcp/30333/ws/p2p/12D3KooWPycx2kKpkkwbzjFSAKdTVFRvntLUKkC7VB3P7aUThmfX
0|subtensor  | 2024-03-19 23:31:46 ⏩ Warping, Downloading state, 8.40 Mib (59 peers), best: #0 (0x2f05…6c03), finalized #0 (0x2f05…6c03), ⬇ 1.4MiB/s ⬆ 42.2kiB/s
0|subtensor  | 2024-03-19 23:31:51 ⏩ Warping, Downloading state, 48.79 Mib (74 peers), best: #0 (0x2f05…6c03), finalized #0 (0x2f05…6c03), ⬇ 4.0MiB/s ⬆ 14.2kiB/s
```

At some point you have to see some line such as the following

```bash
Imported #2596101 (0xfdc2…8016)
```

Be sure the **#xxxxxxxx**, which is the current block, matches the one in [polkadot](https://polkadot.js.org/apps/#/explorer)

<br />

# Uninstallation

To uninstall the local subtensor, run
```bash
pm2 stop subtensor && pm2 delete subtensor
```
