# Third-Party Notices

This document supplements, but does not replace, the license files shipped with HaskMC and its
dependencies. A notice is attribution, not a grant of rights beyond the cited license or terms.

## Pumpkin

HaskMC is derived from [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin). Original Pumpkin source
code is licensed under GNU GPL version 3. Original authorship is preserved in Git history.

The inherited plugin API and plugin WIT definitions are licensed under MIT or Apache-2.0. See:

- `crates/haskmc-plugin-api/LICENSE-MIT`
- `crates/haskmc-plugin-api/LICENSE-APACHE`
- `crates/haskmc-plugin-utils/LICENSE-MIT`
- `crates/haskmc-plugin-utils/LICENSE-APACHE`
- the license files in the `crates/haskmc-plugin-wit` submodule

The MIT notices continue to identify Pumpkin Contributors. HaskMC does not remove or replace that
copyright attribution.

## Minecraft and Mojang/Microsoft material

Minecraft names, game data, and assets remain owned by Mojang Studios and/or Microsoft. HaskMC is
not affiliated with or approved by either company.

The inherited Pumpkin tree includes Minecraft-derived files under `assets/` (including datapacks,
language data, registry data, recipes, loot tables, tags, and structures). It also includes
generated Rust data derived from those inputs. This material is not licensed under HaskMC's GPL
license. Its presence in upstream Pumpkin is under active legal/provenance review:

<https://github.com/Pumpkin-MC/Pumpkin/issues/2472>

The Minecraft EULA and Usage Guidelines do not become an open-source redistribution license merely
because a file is attributed. Anyone redistributing this repository or a compiled binary must make
their own rights assessment and comply with the current terms:

- <https://www.minecraft.net/eula>
- <https://www.minecraft.net/usage-guidelines>

HaskMC's initial rebrand does not add the local copied structure files or decompiled-code
translations that were present in the development workspace. Automated release and container
publishing remains disabled while the inherited asset issue is unresolved.

## Geyser and Bedrock compatibility data

Parts of `assets/bedrock/` originate from or are adapted from GeyserMC material. Geyser's MIT
license is retained at `assets/bedrock/LICENSE-GEYSER`. Some Bedrock-derived data may have separate
underlying rights; see `assets/NOTICE.md` and file-level provenance before redistribution.

## ViaVersion family

Protocol conversion data under `assets/viabackwards/` and `assets/viarewind/` is inherited from the
ViaVersion ecosystem and is subject to the applicable GPLv3 and/or MIT terms and upstream notices.

## Conventional tags

Files under `assets/datapacks/26_2/data/c/tags/` implement conventional/common tag specifications
associated with Fabric and NeoForge and are subject to the applicable Apache-2.0 and/or MIT terms.

## Rust dependencies

HaskMC's resolved Rust dependencies are listed in `Cargo.lock` and retain their own licenses.
Binary distribution is intentionally disabled until a generated dependency-license bundle and a
complete corresponding-source archive are included with every release artifact and container.

## HaskMC artwork

`assets/haskmc-logo.png`, `assets/haskmc-server-icon.png`, and `assets/haskmc-icon.ico` were created
for HaskMC on 2026-08-28 without reference images. They do not intentionally incorporate Pumpkin,
Minecraft, Mojang, or Microsoft artwork.
