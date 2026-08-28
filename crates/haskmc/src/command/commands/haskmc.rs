use haskmc_data::packet::CURRENT_MC_VERSION;
use haskmc_util::text::TextComponent;
use haskmc_util::text::click::ClickEvent;
use haskmc_util::text::color::NamedColor;
use std::borrow::Cow;

use crate::command::CommandResult;
use crate::command::{CommandExecutor, CommandSender, args::ConsumedArgs, tree::CommandTree};

const NAMES: [&str; 4] = ["haskmc", "version", "ver", "pumpkin"];
const DESCRIPTION: &str = "Display information about HaskMC.";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

struct Executor;

impl CommandExecutor for Executor {
    fn execute(
        &self,
        sender: &CommandSender,
        _server: &crate::server::Server,
        _args: &ConsumedArgs,
    ) -> CommandResult {
        let build_profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };

        let message = TextComponent::text(format!(
            "HaskMC {CARGO_PKG_VERSION} ({GIT_HASH}/{build_profile})\nMinecraft {} (protocol {})\n",
            CURRENT_MC_VERSION,
            CURRENT_MC_VERSION.protocol_version()
        ))
        .color_named(NamedColor::Green)
        .add_child(
            TextComponent::text("HaskMC repository")
                .click_event(ClickEvent::OpenUrl {
                    url: Cow::Borrowed("https://github.com/haskbasirat/HaskMC"),
                })
                .color_named(NamedColor::Blue)
                .underlined(),
        )
        .add_child(TextComponent::text(" · independent modified fork of "))
        .add_child(
            TextComponent::text("Pumpkin")
                .click_event(ClickEvent::OpenUrl {
                    url: Cow::Borrowed("https://github.com/Pumpkin-MC/Pumpkin"),
                })
                .color_named(NamedColor::Gold)
                .underlined(),
        )
        .add_child(TextComponent::text(
            "\nNot an official Minecraft product; not approved by or associated with Mojang or Microsoft.",
        ));

        sender.send_message(message);
        Ok(1)
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(Executor)
}

#[cfg(test)]
mod tests {
    use super::NAMES;

    #[test]
    fn legacy_pumpkin_alias_is_retained() {
        assert!(NAMES.contains(&"haskmc"));
        assert!(NAMES.contains(&"pumpkin"));
    }
}
