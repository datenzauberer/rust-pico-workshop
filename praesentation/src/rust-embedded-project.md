# Projektaufbau

<!--
`tree -a -L 2`
-->

```
.
├── .cargo
│   └── config.toml
|── Cargo.toml
├── Embed.toml
├── memory.x
├── README.md
├── src
│   └── main.rs
└── .vscode
    └── settings.json
```

* `.cargo/config` specifies runner, build target
* `Embed.toml` configures `probe-rs`, `gdb`, `rtt`
* `memory.x` describes where the RAM and FLASH are and their sizes
* `build.rs` ensures `memory.x` is in out directory
* `.vscode/settings.json` configures debugger

