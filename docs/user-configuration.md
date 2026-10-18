# User configuration v1

Miilo's shared user directory is `~/.miilo/`, overridden by an absolute
`MIILO_HOME`. It belongs to Miilo across clients, independently of Omarchy.
The native runtime uses existing JSON formats; no TOML conversion is needed.
The protocol schema is `protocol/schemas/user-configuration.schema.json`.
The Rust `UserConfiguration` contract preserves unknown settings and validates
the version and native path. Test it with
`cargo test --manifest-path rust/Cargo.toml --test user_configuration`.
This configuration workflow requires no Python schema-test environment.

```
~/.miilo/
  config.json                         # schemaVersion: 1; activation manifest
  connections/config.json             # existing version-1 endpoint document
  connections/chat.json               # existing explicitly consented chat setup
  connections/local-discovery.json    # dismissed discovery preference
  preferences/70-miilo-fleet.owl.json # existing OWL persistent presentation state
```

The manifest may reference `localConfigFile`, an absolute path to the existing
local observation document. The explicit migration preserves this path, including
service-bound configuration, rather than duplicating enrolled sessions or
rewriting/restarting a collector. Without that field, updated clients use
`MIILO_HOME/local.json`. `MIILO_LOCAL_CONFIG` always wins. Existing endpoint
IDs, provider selections, read-only/action settings and actor aliases retain
their meanings. Quiet/Orbit, motion, personality, pace and colors retain their
existing OWL persistence format. Generated caches are not migrated.

`miilo config paths` is read-only. `init` and `migrate` explicitly copy existing
helper configuration and presentation preferences; `check` validates the root
manifest. Migration never enrolls sessions, connects to endpoints, starts work,
changes credentials, starts services or loads project configuration. Clients
without an activation manifest keep using their old locations. Unknown manifest
versions fail; they are not overwritten or treated as empty configuration.

Directories are user-owned with mode 700 and migrated files mode 600. Migration
keeps originals untouched, refuses symlinks/conflicting destinations and publishes
the activation manifest last. Byte-identical staged files support safe retry.
Stop old UI writers before migration and restart updated clients afterward;
old binaries continue using their old stores. Do not alternate old/new clients.

`MIILO_OWL_CONFIG_DIR` remains an explicit helper override, even after migration.
Migration can copy from it, but the override still wins until the operator removes
it. Snapshot locations, systemd unit configuration and environment overrides are
preserved. No data is implicitly searched under a workspace `.miilo/` directory.

Credential handling remains compatible: existing inline credentials are preserved
inside private files; existing credential-file references stay references. No
keyring integration or credential conversion is claimed. Keyring support is a
separate adapter feature; configuration and diagnostics must never print tokens.
