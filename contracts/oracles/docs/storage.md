# Oracles Contract Storage

## Persistent Storage

| Key | Type | Description |
|-----|------|-------------|
| `DataKey::Admin` | `Address` (instance) | Address that initialized the contract |
| `DataKey::OracleList` | `Vec<Address>` | Ordered list of registered oracle addresses |
| `DataKey::MaxPriceStalenessSeconds` | `u64` (instance) | Maximum accepted age of oracle price data in seconds |

`initialize(admin, max_staleness_seconds)` sets the staleness threshold once and
requires the admin's authorization. Before initialization, health checks use
the default threshold of 3,600 seconds.

## State Diagram

```
EmptyRegistry
   │
   ├── add_oracle(admin, oracle)
   │      ├── requires `capabilities()` to include `CapabilityFlag::GET_PRICE`
   │      └── push_back → NonEmptyRegistry
   │
   ├── remove_oracle(admin, oracle)
   │      └── ItemCount─1 → EmptyRegistry / NonEmptyRegistry
   │
   └── list_oracles()
          └── Reads registry (with TTL bump)
```

## TTL Management

The oracle list key is bumped on every read via `bump_registry_ttl()`:

- **Threshold**: `REGISTRY_TTL_BUMP_THRESHOLD = 120_960` ledgers
- **Extend to**: `REGISTRY_TTL_BUMP_TO = 518_400` ledgers
