# Amnezia

A DPI-proof WireGuard-based VPN protocol.

## Installing (Arch)

Install
- `amneziawg-dkms` and `amneziawg-tools` from AUR. (Keep in mind that their manpages just redirect to WireGuard ones and thus are useless) 
- `systemd-resolvconf` (for awg-quick)

## Setting up (awg-quick)

> :warning:
>
> `systemd-resolved` service must be running on your system for `resolvconf` (`systemd-resolvconf`) command to work.

First, we need:
- Server key pair
- Client key pair
- A pre-shared key

Generate the keys with[^3]:
- On both server and client: `$ wg genkey | tee privatekey | wg pubkey > publickey` (each should have their own)
- On just one machine: `$ wg genpsk > presharedkey` (copypaste it to the other one)

With keys ready, it's time to create our interfaces.

Keep in mind that on a VPN server and clients must have their own distinct IPs. For this example we'll be using:
- `10.10.0.1` for the server
- `10.10.0.2` for the client

For the special Amnezia parameters, consider using a randomizer.

As root, create and modify a file (`/etc/amnezia/amneziawg/<interface>.conf`):

`awg0.conf`[^1][^2]
```ini
[Interface]
# The node's IP address on the VPN network (replace IP with the IP on VPN network for this machine).
Address = IP/24
# Copy this from `privatekey`. Also leaking this is critical and you then must change the keys.
PrivateKey = your_server_private_key
# The port on which the WireGuard interface listens for incoming connections.
ListenPort = 51820
# DNS servers to be used by connected VPN clients.
DNS = 1.1.1.1, 1.0.0.1

# IPtables configuration without which nothing will work. It makes it so the interface can pretend to be sending packets from
# any IPs. No, I do not remember where I got this from, and I'm not willing to search for it.
#
# Make sure to replace `inet` with your network device (do `ip a`, it's probably either `wlan`, `eth`, or something similar).
PostUp = iptables -A FORWARD -i awg0 -j ACCEPT; iptables -t nat -A POSTROUTING -o inet -j MASQUERADE; ip6tables -A FORWARD -i awg0 -j ACCEPT; ip6tables -t nat -A POSTROUTING -o inet -j MASQUERADE
PostDown = iptables -D FORWARD -i awg0 -j ACCEPT; iptables -t nat -D POSTROUTING -o inet -j MASQUERADE; ip6tables -D FORWARD -i awg0 -j ACCEPT; ip6tables -t nat -D POSTROUTING -o inet -j MASQUERADE

# Amnezia parameters.
# Those must be exactly the same on both devices.

# (Optional) Traffic obfuscation parameters to help bypass DPI.
# Range: 0-10. Number of junk packets following I1-I5.
Jc = 12
# Jmin, Jmax, Range: 64-1024. Size range for random junk packets.
Jmin = 623
Jmax = 1002
# S1-S3, Range: 0-64
S1 = 43
S2 = 21
S3 = 46
# S4 (isn't here for some reason), Range: 0-32.
S4 = 31
# H1-H4, range: 0-4 294 967 295
H1 = 1953034736
H2 = 752945292
H3 = 3945748733
H4 = 1666444888

[Peer]
# Copypaste from the other machine's `publickey`.
PublicKey = client_public_key
# Copypaste from `presharedkey`. Make sure this doesn't leak and only use it for one peer. If you want to have several devices, create a new pre-shared key.
#
# This is not required, but is probably more secure than not having it.
PresharedKey = your_preshared_key
# IPs that will be bound to this interface.
# On the server you can just put the peer's VPN IP.
# On the client, you may do the same, or do `0.0.0.0/0, ::/0` if you want to proxy all trafic.
#
# Usually this option is labeled vaguely in all docs, but this is what it actually does! So
# make sure to not accidentally set it to all IPs on your server, cuz otherwise it will become
# forever inaccessible.
AllowedIPs = IP/32
# The IP to which this node will send packets to.
#
# This is optional on the server, but is required on the client.
Endpoint = 0.0.0.0:51820
# (Optional) Keeping the connection alive, especially for NAT scenarios.
PersistentKeepalive = 25
```

Then on both sides you can do `# awg-quick up <interface>`.

(p.s. you can also pass an absolute path)

Important to note that manually routing via the new awg device will only work if those IP are specified in `AllowedIPs`.
If you want to be able to choose traffic from what applications you want to pass through awg, consider trying [linux/netns|Network Namespaces].

[^1]: https://ithy.com/article/amneziawg-configuration-ah56mxc2
[^2]: https://docs.amnezia.org/documentation/amnezia-wg#configuration-parameters
[^3]: https://www.wireguard.com/quickstart/
