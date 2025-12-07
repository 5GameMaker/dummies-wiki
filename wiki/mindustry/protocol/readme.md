# Mindustry Protocol

## Types

```bytedoc
TypeIOString:
(byte(exists))
{if exists != 0, (u16be(len))(byte[len]: cesu8 string)}

NetworkIOString:
(u8(len): length)
(byte[len]: java cesu8 string)
```

## Gamemodes

- `0`: `survival`
- `1`: `sandbox`
- `2`: `attack`
- `3`: `pvp`
- `4`: `editor`

## Version types

- `official`
- `bleeding edge`
- `custom`

## Ping

```bytedoc
client -> server (udp): (byte[2] {254, 1}: discover host)
client <- server (udp): (Host)

Host (byte[..512]):
(NetworkIOString: server name)
(NetworkIOString: map name)
(i32be: player count)
(i32be: current wave)
(i32be: server build version)
(NetworkIOString: version type (see [[mindustry/protocol/readme#Version types]]))
(byte: server gamemode (see [[mindustry/protocol/readme#Gamemodes]]))
(i32be: max players (if 0, no player limit))
(NetworkIOString: server description)
(NetworkIOString?: gamemode name, if none or empty uses the name of a gamemode)
```

## Connecting

```bytedoc
# Login stage
client -> server (tcp): (ack)
client <- server (tcp): (Packet<FrameworkMessage.RegisterTCP>)
client -> server (udp): (Packet<FrameworkMessage.RegisterUDP>)
client <- server (tcp): (Packet<FrameworkMessage.RegisterUDP>)
client -> server (tcp): (Packet<ConnectPacket>)

Lz4<T>:
(u32be: length of data (excluding length and compression + 1) as send over the network)
(byte: 0 if uncompressed, otherwise 1)
(byte[length](data): lz4 block format data)

Packet<T>:
(i16be(len)?: (TCP packets only!) amount of bytes left to receive)
(byte(type): packet type)
{if type = 254, (T), else (Lz4<T>)}

FrameworkMessage: (Packet.type=254)
(FrameworkMessage.?: packet data)

FrameworkMessage.Ping: (Packet.type=254)
(byte = 0: id)
(i32be: ping id)
(byte: 1 if reply, otherwise 0)

FrameworkMessage.DiscoverHost: (Packet.type=254)
(byte = 1: id)

FrameworkMessage.KeepAlive: (id=254)
(byte = 2: id)

FrameworkMessage.RegisterUDP: (id=254)
(byte = 3: id)
(i32be: connection id)

FrameworkMessage.RegisterTCP: (id=254)
(byte = 4: id)
(i32be: connection id)

ConnectPacket: (Packet.type=3)
(i32be: version)
(TypeIOString: version type)
(TypeIOString: username)
(TypeIOString: locale)
(TypeIOString: usid)
(u8[16]: uuid)
(byte: 1 if mobile, otherwise 0)
(i32be: color)
(u8(mod_count): mod count)
(TypeIOString[mod_count]: mods)

KickPacket: << v146: Packet.type=44, v151: Packet.type=52 >>
```

## Entity snapshots (entity syncing)

```bytedoc
EntitySnapshotPacket:
(i32be: entity id)
(byte: entity class id)
(dyn ?: entity data)
```

For entity data, see each entity's corresponding wiki entry (save data).

## Reference

- <https://github.com/5GameMaker/MindustryHotfixv7/blob/master/core/src/mindustry/core/NetServer.java#L914-L974>
- <https://github.com/5GameMaker/MindustryHotfixv7/blob/master/core/src/mindustry/io/TypeIO.java>
