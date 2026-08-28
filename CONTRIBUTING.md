# Contributing to HaskMC

Thank you for helping improve HaskMC. Search the
[issue tracker](https://github.com/haskbasirat/HaskMC/issues) before opening a report or pull
request, and explain what changed, why it is needed, and any known limitations.

## Development setup

```shell
git clone --recurse-submodules https://github.com/haskbasirat/HaskMC.git
cd HaskMC
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

HaskMC uses strict Clippy settings. CPU-intensive work should use Rayon's thread pool without
blocking Tokio workers; `haskmc_world::level::Level::fetch_chunks` is an example.

## Required provenance

Contributions must be original or based on sources whose license permits inclusion in this GPLv3
project. Every non-original data/code contribution must identify its exact source, revision, and
license in the pull request.

Do not submit:

- decompiled or mechanically translated Minecraft client/server code;
- Minecraft client/server JARs, class files, copied NBT structures, assets, or reference worlds;
- generated files made from an undocumented or unlicensed source;
- code or artwork copied from another project without retaining its required license and notice.

Behavioral compatibility work should be implemented from public specifications, independently
written behavioral notes, or black-box tests. Keep a clean-room record when a feature is informed
by observed Minecraft behavior. A change of names or formatting does not make copied code original.

## Pull-request checklist

- Use a focused title and describe the change and its impact.
- Link related issues and disclose source/provenance information.
- Add or update tests for behavior changes.
- Run formatting, Clippy, and the relevant tests.
- Update documentation and third-party notices where necessary.
- Do not remove Pumpkin authorship, Git history, GPL notices, or dual-license notices.

HaskMC is an independent modified fork of
[Pumpkin](https://github.com/Pumpkin-MC/Pumpkin). See [FORK_NOTICE.md](FORK_NOTICE.md) and
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) before contributing.
