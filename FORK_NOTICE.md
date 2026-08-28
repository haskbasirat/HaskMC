# HaskMC Fork Notice

HaskMC is an independent modified fork of Pumpkin:

<https://github.com/Pumpkin-MC/Pumpkin>

The HaskMC rebrand began on 2026-08-28 from Pumpkin commit:

`9e9f1b5784019e18574936684902bc34067d1602`

Pumpkin and its original portions remain copyright their respective authors and contributors.
HaskMC modifications are copyright their respective HaskMC contributors. The preserved Git
history is the authoritative record of authorship and modification history.

The HaskMC server is distributed under GNU General Public License version 3 only. See
[`LICENSE`](LICENSE). The inherited plugin API and plugin WIT definitions are separately
dual-licensed under MIT or Apache-2.0; their original license files and Pumpkin Contributors
copyright notices remain intact.

HaskMC is published and maintained by [Hask Basirat](https://github.com/haskbasirat). Contact the
maintainer through the [HaskMC issue tracker](https://github.com/haskbasirat/HaskMC/issues).

HaskMC is not affiliated with, endorsed by, or sponsored by PumpkinMC, Mojang, or Microsoft.

**NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

## Compatibility names retained from Pumpkin

The following identifiers deliberately retain the Pumpkin name because changing them would break
existing plugins, saved worlds, or third-party marketplace integration:

- the `pumpkin:plugin@0.1.0` WebAssembly component namespace;
- the `PUMPKIN_API_VERSION` legacy native-plugin symbol;
- `pumpkin.metadata`, `PumpkinMetadata`, and Pumpkin Marketplace endpoints;
- reads of the legacy `PumpkinCustomData` NBT key;
- the `/pumpkin` legacy command alias and legacy `pumpkin:` identifier helpers;
- the upstream `pumpkin-plugin-wit` Git URL.

Minecraft identifiers containing “pumpkin” describe Minecraft's pumpkin blocks, items, recipes,
tags, or mechanics and are not HaskMC/Pumpkin project branding.

## Modification summary

The initial HaskMC commit changes project branding, Cargo package and crate names, executable and
container paths, configuration names, user-visible messages, project documentation, and CI/release
names. It adds compatibility fallbacks for existing configuration and world metadata and replaces
the inherited project artwork with an original HaskMC mark.

Local parity changes associated with decompiled Minecraft sources and copied local game assets
were not included in the public HaskMC branch.
