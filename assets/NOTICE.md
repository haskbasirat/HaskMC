# Third-Party Assets and Data Notice

This directory contains inherited data files, protocol mappings, tests, and game-derived assets.
They are not all licensed under the HaskMC/Pumpkin GPL license. This notice records provenance; it
does not grant rights that the identified owner or license has not granted.

## 1. Minecraft assets and data

- **Locations**: `assets/datapacks/`, `assets/loot_table/`, Minecraft registry and language files
  under `assets/`, structure/template data consumed by `crates/haskmc-world`, and generated Rust
  data derived from those inputs.
- **Owner**: Mojang Studios and/or Microsoft Corporation, where copyrightable Minecraft content is
  present.
- **Status**: inherited from Pumpkin and not licensed under GPLv3. Redistribution status is under
  review in <https://github.com/Pumpkin-MC/Pumpkin/issues/2472>.
- **Terms**: <https://www.minecraft.net/eula> and
  <https://www.minecraft.net/usage-guidelines>.

Attribution does not turn Minecraft game files into open-source content or establish permission to
redistribute them. No local decompiled sources, game JARs, reference worlds, or newly copied
structure files are part of the HaskMC rebrand commit.

## 2. Bedrock and Geyser mappings

- **Location**: `assets/bedrock/`
- **Source attribution**: GeyserMC and contributors where identified by file history/comments.
- **License**: MIT for Geyser-owned contributions; see `assets/bedrock/LICENSE-GEYSER`.

Some Bedrock-derived data may carry separate underlying Mojang/Microsoft rights or incomplete
file-level provenance. The Geyser MIT license covers only material its licensors had authority to
license.

## 3. Protocol version translation

- **Locations**: `assets/viabackwards/`, `assets/viarewind/`
- **Projects**: ViaVersion, ViaBackwards, and ViaRewind contributors.
- **Licenses**: applicable GPLv3 and/or MIT terms and upstream notices.

## 4. Conventional tags (`c:`)

- **Location**: `assets/datapacks/26_2/data/c/tags/`
- **Standard**: Fabric and NeoForge conventional/common tags.
- **Licenses**: applicable Apache-2.0 and/or MIT terms.

## 5. Source-code licenses

- Original Pumpkin server source remains GNU GPL version 3 only, with authorship preserved in Git.
- HaskMC server modifications are GNU GPL version 3 only; see the root `LICENSE`.
- `haskmc-plugin-api`, `haskmc-plugin-utils`, and the inherited plugin-WIT submodule are dual-
  licensed under MIT or Apache-2.0, with original Pumpkin Contributors notices retained.

See the root `FORK_NOTICE.md` and `THIRD_PARTY_NOTICES.md` for the complete fork notice and current
distribution policy.
