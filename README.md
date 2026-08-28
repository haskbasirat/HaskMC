<div align="center">

<img src="assets/haskmc-logo.png" alt="HaskMC logo" width="220">

# HaskMC

[![CI](https://github.com/haskbasirat/HaskMC/actions/workflows/rust.yml/badge.svg)](https://github.com/haskbasirat/HaskMC/actions/workflows/rust.yml)
[![License: GPLv3 only](https://img.shields.io/badge/License-GPLv3--only-yellow.svg)](LICENSE)

</div>

HaskMC is an independent, modified fork of [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin),
a high-performance Minecraft-compatible server written in Rust. This rebrand is based on
Pumpkin commit [`9e9f1b5784019e18574936684902bc34067d1602`](https://github.com/Pumpkin-MC/Pumpkin/commit/9e9f1b5784019e18574936684902bc34067d1602)
and preserves the upstream Git history and contributor attribution.

> [!IMPORTANT]
> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
>
> HaskMC is not affiliated with, endorsed by, or sponsored by PumpkinMC. “Pumpkin” refers
> to the upstream open-source project and is retained where attribution or compatibility
> requires it.

Publisher and maintainer: [Hask Basirat](https://github.com/haskbasirat). Project questions
and notices can be submitted through the [HaskMC issue tracker](https://github.com/haskbasirat/HaskMC/issues).

## Project status

HaskMC is under active development. It aims to retain Pumpkin's performance, Java and Bedrock
protocol support, vanilla-compatible mechanics, configurability, and plugin system while
developing under a distinct identity.

The initial HaskMC publication deliberately excludes local parity work derived from decompiled
Minecraft code and copied local game assets. That work remains quarantined outside this public
branch pending a clean-room rewrite or explicit legal clearance.

## Quick start

Requirements are listed in [`rust-toolchain.toml`](rust-toolchain.toml).

```shell
git clone --recurse-submodules https://github.com/haskbasirat/HaskMC.git
cd HaskMC
cargo run --release --package haskmc
```

HaskMC writes configuration to `haskmc.toml`. If only a legacy `pumpkin.toml` exists, it is
loaded once and migrated to `haskmc.toml` without deleting the original file.

## Compatibility retained intentionally

- `/haskmc` is the primary information command; `/pumpkin` remains a legacy alias.
- New custom world data uses `HaskMCCustomData`; existing `PumpkinCustomData` is still read.
- HaskMC's own resource namespace is `haskmc:`; the legacy `pumpkin:` helpers remain available.
- The plugin WIT ABI remains `pumpkin:plugin@0.1.0` so existing Pumpkin plugins are not
  needlessly broken. Marketplace metadata and signature names also remain Pumpkin-compatible.
- Minecraft gameplay identifiers such as `minecraft:pumpkin`, `pumpkin_stem`, and
  `carved_pumpkin` are game data, not project branding, and therefore remain unchanged.

## Licensing and attribution

- The HaskMC server and the original Pumpkin server portions are distributed under
  [GNU GPL version 3 only](LICENSE). Source and modification history are preserved.
- The plugin API and inherited plugin WIT definitions remain dual-licensed under MIT or
  Apache-2.0; their original copyright notices are retained.
- [FORK_NOTICE.md](FORK_NOTICE.md) records the fork origin, base revision, authorship, and
  compatibility exceptions.
- [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and
  [assets/NOTICE.md](assets/NOTICE.md) identify separately licensed or proprietary material.

### Important asset warning

The upstream Pumpkin history currently contains Minecraft-derived data and game assets whose
redistribution status is under active review in
[Pumpkin issue #2472](https://github.com/Pumpkin-MC/Pumpkin/issues/2472). Those files are not
covered by HaskMC's GPL license, and attribution does not grant permission to redistribute them.
Automated binary, container, and nightly publishing is disabled in HaskMC until the corresponding
source and asset-compliance process is resolved. See the notices before redistributing builds.

No legal process can guarantee that a repository will never receive a complaint or takedown.
HaskMC's policy is to preserve attribution, avoid copied/decompiled additions, respond promptly
to rights-holder notices, and remove or replace material whose redistribution cannot be verified.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Contributions must be original or based on clearly
licensed sources. Do not submit decompiled Minecraft code, copied game assets, server/client JARs,
or generated output whose provenance cannot be documented.
