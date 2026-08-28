use wasmtime::component::Resource;

use crate::{
    command::{
        args::{
            GetClientSideArgParser,
            block::{BlockArgumentConsumer, BlockPredicateArgumentConsumer},
            bool::BoolArgConsumer,
            bounded_num::{BoundedNumArgumentConsumer, ToFromNumber},
            difficulty::DifficultyArgumentConsumer,
            entities::EntitiesArgumentConsumer,
            entity::EntityArgumentConsumer,
            entity_anchor::EntityAnchorArgumentConsumer,
            gamemode::GamemodeArgumentConsumer,
            message::MsgArgConsumer,
            players::PlayersArgumentConsumer,
            position_2d::Position2DArgumentConsumer,
            position_3d::Position3DArgumentConsumer,
            position_block::BlockPosArgumentConsumer,
            resource::item::{ItemArgumentConsumer, ItemPredicateArgumentConsumer},
            resource_location::ResourceLocationArgumentConsumer,
            rotation::RotationArgumentConsumer,
            simple::SimpleArgConsumer,
            textcomponent::TextComponentArgConsumer,
            time::TimeArgumentConsumer,
        },
        tree::{
            CommandTree,
            builder::{NonLeafNodeBuilder, argument, literal},
        },
    },
    plugin::loader::wasm::wasm_host::{
        state::{
            CommandNodeResource, CommandResource, CommandSenderResource, ConsumedArgsResource,
            PluginHostState, ServerResource, TextComponentResource,
        },
        wit::v0_1::{
            commands::executor::{WasmCommandExecutor, WasmCommandSuggestionProvider},
            pumpkin::{
                self,
                plugin::{
                    command::{
                        Arg, ArgumentType, Command, CommandNode, CommandSender, CommandSenderType,
                        ConsumedArgs, PermissionLevel, StringType,
                    },
                    common::{BlockPos as WitBlockPos, Locale, Position},
                    player::Player,
                    server::Server,
                    text::TextComponent,
                    world::World,
                },
            },
        },
    },
};

pub mod executor;

impl PluginHostState {
    fn get_command_mut(
        &mut self,
        res: &Resource<Command>,
    ) -> wasmtime::Result<&mut CommandResource> {
        self.resource_table
            .get_mut::<CommandResource>(&Resource::new_own(res.rep()))
            .map_err(wasmtime::Error::from)
    }
    fn get_node_mut(
        &mut self,
        res: &Resource<CommandNode>,
    ) -> wasmtime::Result<&mut CommandNodeResource> {
        self.resource_table
            .get_mut::<CommandNodeResource>(&Resource::new_own(res.rep()))
            .map_err(wasmtime::Error::from)
    }
    fn take_node(&mut self, res: &Resource<CommandNode>) -> wasmtime::Result<CommandNodeResource> {
        self.resource_table
            .delete::<CommandNodeResource>(Resource::new_own(res.rep()))
            .map_err(wasmtime::Error::from)
    }
    fn get_sender_res(
        &self,
        res: &Resource<CommandSender>,
    ) -> wasmtime::Result<&CommandSenderResource> {
        self.resource_table
            .get::<CommandSenderResource>(&Resource::new_own(res.rep()))
            .map_err(wasmtime::Error::from)
    }
    fn get_sender_mut(
        &mut self,
        res: &Resource<CommandSender>,
    ) -> wasmtime::Result<&mut CommandSenderResource> {
        self.resource_table
            .get_mut::<CommandSenderResource>(&Resource::new_own(res.rep()))
            .map_err(wasmtime::Error::from)
    }
}

impl pumpkin::plugin::command::Host for PluginHostState {}

impl pumpkin::plugin::command::HostConsumedArgs for PluginHostState {
    #[expect(clippy::too_many_lines)]
    async fn get_value(
        &mut self,
        consumed_args: Resource<ConsumedArgs>,
        key: String,
    ) -> wasmtime::Result<Arg> {
        use crate::plugin::loader::wasm::wasm_host::args::OwnedArg;

        let resource = self
            .resource_table
            .get::<ConsumedArgsResource>(&Resource::new_own(consumed_args.rep()))
            .map_err(wasmtime::Error::from)?;

        let Some(owned_arg) = resource.provider.get(&key).cloned() else {
            return Ok(Arg::Simple(String::new()));
        };

        Ok(match owned_arg {
            OwnedArg::Simple(s) => Arg::Simple(s),
            OwnedArg::Msg(s) => Arg::Msg(s),
            OwnedArg::Bool(b) => Arg::Bool(b),
            OwnedArg::Item(s) => Arg::Item(s),
            OwnedArg::ItemPredicate(s) => Arg::ItemPredicate(s),
            OwnedArg::ResourceLocation(s) => Arg::ResourceLocation(s),
            OwnedArg::Block(s) => Arg::Block(s),
            OwnedArg::BlockPredicate(s) => Arg::BlockPredicate(s),
            OwnedArg::Time(t) => Arg::Time(t),
            OwnedArg::Num(n) => {
                use crate::command::args::bounded_num::{NotInBounds, Number};
                let convert_num = |n: Number| match n {
                    Number::F64(v) => pumpkin::plugin::command::Number::Float64(v),
                    Number::F32(v) => pumpkin::plugin::command::Number::Float32(v),
                    Number::I32(v) => pumpkin::plugin::command::Number::Int32(v),
                    Number::I64(v) => pumpkin::plugin::command::Number::Int64(v),
                };
                Arg::Num(n.map(convert_num).map_err(|e| match e {
                    NotInBounds::LowerBound(a, b) => {
                        pumpkin::plugin::command::NotInBounds::LowerBound((
                            convert_num(a),
                            convert_num(b),
                        ))
                    }
                    NotInBounds::UpperBound(a, b) => {
                        pumpkin::plugin::command::NotInBounds::UpperBound((
                            convert_num(a),
                            convert_num(b),
                        ))
                    }
                }))
            }
            OwnedArg::BlockPos(p) => Arg::BlockPos(WitBlockPos {
                x: p.0.x,
                y: p.0.y,
                z: p.0.z,
            }),
            OwnedArg::Pos3D(v) => Arg::Pos3d((v.x, v.y, v.z)),
            OwnedArg::Pos2D(v) => Arg::Pos2d((v.x, v.y)),
            OwnedArg::Rotation(a, b, c, d) => Arg::Rotation((a, b, c, d)),
            OwnedArg::GameMode(g) => Arg::Gamemode(match g {
                haskmc_util::GameMode::Survival => pumpkin::plugin::common::GameMode::Survival,
                haskmc_util::GameMode::Creative => pumpkin::plugin::common::GameMode::Creative,
                haskmc_util::GameMode::Adventure => pumpkin::plugin::common::GameMode::Adventure,
                haskmc_util::GameMode::Spectator => pumpkin::plugin::common::GameMode::Spectator,
            }),
            OwnedArg::Difficulty(d) => Arg::Difficulty(match d {
                haskmc_util::Difficulty::Peaceful => pumpkin::plugin::server::Difficulty::Peaceful,
                haskmc_util::Difficulty::Easy => pumpkin::plugin::server::Difficulty::Easy,
                haskmc_util::Difficulty::Normal => pumpkin::plugin::server::Difficulty::Normal,
                haskmc_util::Difficulty::Hard => pumpkin::plugin::server::Difficulty::Hard,
            }),
            OwnedArg::Players(players) => {
                let mut resources = Vec::new();
                for p in players {
                    if let Ok(r) = self.add_player(p) {
                        resources.push(r);
                    }
                }
                Arg::Players(resources)
            }
            OwnedArg::Particle(p) => Arg::Particle(format!("{p:?}")),
            OwnedArg::TextComponent(t) => {
                let r = self
                    .resource_table
                    .push(TextComponentResource { provider: t })
                    .map_err(wasmtime::Error::from)?;
                Arg::TextComponent(wasmtime::component::Resource::new_own(r.rep()))
            }
            OwnedArg::BossbarColor(c) => Arg::BossbarColor(match c {
                crate::world::bossbar::BossbarColor::Pink => {
                    pumpkin::plugin::command::BossbarColor::Pink
                }
                crate::world::bossbar::BossbarColor::Blue => {
                    pumpkin::plugin::command::BossbarColor::Blue
                }
                crate::world::bossbar::BossbarColor::Red => {
                    pumpkin::plugin::command::BossbarColor::Red
                }
                crate::world::bossbar::BossbarColor::Green => {
                    pumpkin::plugin::command::BossbarColor::Green
                }
                crate::world::bossbar::BossbarColor::Yellow => {
                    pumpkin::plugin::command::BossbarColor::Yellow
                }
                crate::world::bossbar::BossbarColor::Purple => {
                    pumpkin::plugin::command::BossbarColor::Purple
                }
                crate::world::bossbar::BossbarColor::White => {
                    pumpkin::plugin::command::BossbarColor::White
                }
            }),
            OwnedArg::BossbarStyle(s) => Arg::BossbarStyle(match s {
                crate::world::bossbar::BossbarDivisions::NoDivision => {
                    pumpkin::plugin::command::BossbarStyle::NoDivision
                }
                crate::world::bossbar::BossbarDivisions::Notches6 => {
                    pumpkin::plugin::command::BossbarStyle::Notches6
                }
                crate::world::bossbar::BossbarDivisions::Notches10 => {
                    pumpkin::plugin::command::BossbarStyle::Notches10
                }
                crate::world::bossbar::BossbarDivisions::Notches12 => {
                    pumpkin::plugin::command::BossbarStyle::Notches12
                }
                crate::world::bossbar::BossbarDivisions::Notches20 => {
                    pumpkin::plugin::command::BossbarStyle::Notches20
                }
            }),
            OwnedArg::SoundCategory(s) => Arg::SoundCategory(match s {
                haskmc_data::sound::SoundCategory::Master
                | haskmc_data::sound::SoundCategory::Ui => {
                    pumpkin::plugin::command::SoundCategory::Master
                }
                haskmc_data::sound::SoundCategory::Music => {
                    pumpkin::plugin::command::SoundCategory::Music
                }
                haskmc_data::sound::SoundCategory::Records => {
                    pumpkin::plugin::command::SoundCategory::Records
                }
                haskmc_data::sound::SoundCategory::Weather => {
                    pumpkin::plugin::command::SoundCategory::Weather
                }
                haskmc_data::sound::SoundCategory::Blocks => {
                    pumpkin::plugin::command::SoundCategory::Blocks
                }
                haskmc_data::sound::SoundCategory::Hostile => {
                    pumpkin::plugin::command::SoundCategory::Hostile
                }
                haskmc_data::sound::SoundCategory::Neutral => {
                    pumpkin::plugin::command::SoundCategory::Neutral
                }
                haskmc_data::sound::SoundCategory::Players => {
                    pumpkin::plugin::command::SoundCategory::Players
                }
                haskmc_data::sound::SoundCategory::Ambient => {
                    pumpkin::plugin::command::SoundCategory::Ambient
                }
                haskmc_data::sound::SoundCategory::Voice => {
                    pumpkin::plugin::command::SoundCategory::Voice
                }
            }),
            OwnedArg::DamageType(d) => Arg::DamageType(d.message_id.to_string()),
            OwnedArg::Effect(e) => Arg::Effect(e.minecraft_name.to_string()),
            OwnedArg::Enchantment(e) => Arg::Enchantment(e.name.to_string()),
            OwnedArg::Advancement(a) => Arg::Advancement(a.to_string()),
            OwnedArg::EntityAnchor(a) => Arg::EntityAnchor(match a {
                crate::command::args::EntityAnchor::Eyes => {
                    pumpkin::plugin::command::EntityAnchor::Eyes
                }
                crate::command::args::EntityAnchor::Feet => {
                    pumpkin::plugin::command::EntityAnchor::Feet
                }
            }),
            // These types don't have direct WIT resource mappings yet
            OwnedArg::Entities(_)
            | OwnedArg::Entity(_)
            | OwnedArg::GameProfiles(_)
            | OwnedArg::CommandTree(_) => Arg::Simple(String::new()),
        })
    }

    async fn drop(&mut self, rep: Resource<ConsumedArgs>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<ConsumedArgsResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}

impl pumpkin::plugin::command::HostCommand for PluginHostState {
    async fn new(
        &mut self,
        names: Vec<String>,
        description: String,
    ) -> wasmtime::Result<Resource<Command>> {
        self.add_command(CommandTree::new(names, description))
            .map_err(|_| wasmtime::Error::msg("Failed to add command resource"))
    }

    async fn then(
        &mut self,
        command: Resource<Command>,
        node: Resource<CommandNode>,
    ) -> wasmtime::Result<()> {
        let node_data = self.take_node(&node)?;
        let command_res = self.get_command_mut(&command)?;
        command_res.provider = command_res.provider.clone().then(node_data.provider);
        Ok(())
    }

    async fn execute_with_handler_id(
        &mut self,
        command: Resource<Command>,
        handler_id: u32,
    ) -> wasmtime::Result<()> {
        let plugin = self
            .plugin
            .as_ref()
            .and_then(std::sync::Weak::upgrade)
            .ok_or_else(|| wasmtime::Error::msg("Plugin dropped"))?;
        let server = self
            .server
            .clone()
            .ok_or_else(|| wasmtime::Error::msg("Server not initialized"))?;

        let executor = WasmCommandExecutor {
            handler_id,
            plugin,
            server,
        };
        let command_res = self.get_command_mut(&command)?;
        command_res.provider = command_res.provider.clone().execute(executor);
        Ok(())
    }

    async fn drop(&mut self, rep: Resource<Command>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<CommandResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}

impl pumpkin::plugin::command::HostCommandSender for PluginHostState {
    async fn get_command_sender_type(
        &mut self,
        _res: Resource<CommandSender>,
    ) -> wasmtime::Result<CommandSenderType> {
        Err(wasmtime::Error::msg(
            "get_command_sender_type not implemented",
        ))
    }

    async fn get_name(&mut self, sender: Resource<CommandSender>) -> wasmtime::Result<String> {
        Ok(self.get_sender_res(&sender)?.provider.to_string())
    }

    async fn send_message(
        &mut self,
        sender: Resource<CommandSender>,
        text: Resource<TextComponent>,
    ) -> wasmtime::Result<()> {
        let component = self
            .resource_table
            .get::<TextComponentResource>(&Resource::new_own(text.rep()))?
            .provider
            .clone();
        self.get_sender_res(&sender)?
            .provider
            .send_message(component);
        Ok(())
    }

    async fn send_system_message(
        &mut self,
        sender: Resource<CommandSender>,
        text: Resource<TextComponent>,
    ) -> wasmtime::Result<()> {
        let component = self
            .resource_table
            .get::<TextComponentResource>(&Resource::new_own(text.rep()))?
            .provider
            .clone();
        self.get_sender_res(&sender)?
            .provider
            .send_message(component);
        Ok(())
    }

    async fn send_error(
        &mut self,
        sender: Resource<CommandSender>,
        text: Resource<TextComponent>,
    ) -> wasmtime::Result<()> {
        let component = self
            .resource_table
            .get::<TextComponentResource>(&Resource::new_own(text.rep()))?
            .provider
            .clone();
        self.get_sender_res(&sender)?
            .provider
            .send_message(component.color(haskmc_util::text::color::Color::Named(
                haskmc_util::text::color::NamedColor::Red,
            )));
        Ok(())
    }

    async fn set_success_count(
        &mut self,
        sender: Resource<CommandSender>,
        count: i32,
    ) -> wasmtime::Result<()> {
        self.get_sender_mut(&sender)?
            .provider
            .set_success_count(count as u32);
        Ok(())
    }

    async fn is_player(&mut self, sender: Resource<CommandSender>) -> wasmtime::Result<bool> {
        Ok(matches!(
            self.get_sender_res(&sender)?.provider,
            crate::command::CommandSender::Player(_)
        ))
    }

    async fn is_console(&mut self, sender: Resource<CommandSender>) -> wasmtime::Result<bool> {
        Ok(matches!(
            self.get_sender_res(&sender)?.provider,
            crate::command::CommandSender::Console | crate::command::CommandSender::Rcon(_)
        ))
    }

    async fn as_player(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<Option<Resource<Player>>> {
        if let crate::command::CommandSender::Player(player) =
            &self.get_sender_res(&sender)?.provider
        {
            Ok(Some(self.add_player(player.clone()).map_err(|_| {
                wasmtime::Error::msg("Failed to add player resource")
            })?))
        } else {
            Ok(None)
        }
    }

    async fn permission_level(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<PermissionLevel> {
        Ok(
            match self.get_sender_res(&sender)?.provider.permission_lvl() {
                haskmc_util::PermissionLvl::Zero => PermissionLevel::Zero,
                haskmc_util::PermissionLvl::One => PermissionLevel::One,
                haskmc_util::PermissionLvl::Two => PermissionLevel::Two,
                haskmc_util::PermissionLvl::Three => PermissionLevel::Three,
                haskmc_util::PermissionLvl::Four => PermissionLevel::Four,
            },
        )
    }

    async fn has_permission_level(
        &mut self,
        sender: Resource<CommandSender>,
        level: PermissionLevel,
    ) -> wasmtime::Result<bool> {
        let required = match level {
            PermissionLevel::Zero => haskmc_util::PermissionLvl::Zero,
            PermissionLevel::One => haskmc_util::PermissionLvl::One,
            PermissionLevel::Two => haskmc_util::PermissionLvl::Two,
            PermissionLevel::Three => haskmc_util::PermissionLvl::Three,
            PermissionLevel::Four => haskmc_util::PermissionLvl::Four,
        };
        Ok(self.get_sender_res(&sender)?.provider.permission_lvl() >= required)
    }

    async fn has_permission(
        &mut self,
        sender: Resource<CommandSender>,
        server: Resource<Server>,
        node: String,
    ) -> wasmtime::Result<bool> {
        let sender_provider = &self.get_sender_res(&sender)?.provider;
        let server_provider = &self
            .resource_table
            .get::<ServerResource>(&Resource::new_own(server.rep()))?
            .provider;
        Ok(sender_provider.has_permission(server_provider, &node))
    }

    async fn position(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<Option<Position>> {
        Ok(self
            .get_sender_res(&sender)?
            .provider
            .position()
            .map(|p| (p.x, p.y, p.z)))
    }

    async fn world(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<Option<Resource<World>>> {
        if let Some(world) = self.get_sender_res(&sender)?.provider.world() {
            Ok(Some(self.add_world(world).map_err(|_| {
                wasmtime::Error::msg("Failed to add world resource")
            })?))
        } else {
            Ok(None)
        }
    }

    async fn get_locale(&mut self, sender: Resource<CommandSender>) -> wasmtime::Result<Locale> {
        Ok(map_util_locale_to_wit(
            self.get_sender_res(&sender)?.provider.get_locale(),
        ))
    }

    async fn should_receive_feedback(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<bool> {
        Ok(self
            .get_sender_res(&sender)?
            .provider
            .should_receive_feedback())
    }

    async fn should_broadcast_console_to_ops(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<bool> {
        Ok(self
            .get_sender_res(&sender)?
            .provider
            .should_broadcast_console_to_ops())
    }

    async fn should_track_output(
        &mut self,
        sender: Resource<CommandSender>,
    ) -> wasmtime::Result<bool> {
        Ok(self.get_sender_res(&sender)?.provider.should_track_output())
    }

    async fn drop(&mut self, rep: Resource<CommandSender>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<CommandSenderResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}

impl pumpkin::plugin::command::HostCommandNode for PluginHostState {
    async fn literal(&mut self, name: String) -> wasmtime::Result<Resource<CommandNode>> {
        self.add_command_node(literal(name))
            .map_err(|_| wasmtime::Error::msg("Failed to add literal node"))
    }

    async fn argument(
        &mut self,
        name: String,
        arg_type: ArgumentType,
    ) -> wasmtime::Result<Resource<CommandNode>> {
        let node = match arg_type {
            ArgumentType::Bool => argument(name, BoolArgConsumer),
            ArgumentType::Float((min, max)) => build_bounded_node::<f32>(name, min, max),
            ArgumentType::Double((min, max)) => build_bounded_node::<f64>(name, min, max),
            ArgumentType::Integer((min, max)) => build_bounded_node::<i32>(name, min, max),
            ArgumentType::Long((min, max)) => build_bounded_node::<i64>(name, min, max),
            ArgumentType::String(st) => match st {
                StringType::SingleWord | StringType::Quotable => argument(name, SimpleArgConsumer),
                StringType::Greedy => argument(name, MsgArgConsumer),
            },
            ArgumentType::Entities => argument(name, EntitiesArgumentConsumer),
            ArgumentType::Entity => argument(name, EntityArgumentConsumer),
            ArgumentType::Players | ArgumentType::GameProfile => {
                argument(name, PlayersArgumentConsumer)
            }
            ArgumentType::BlockPos => argument(name, BlockPosArgumentConsumer),
            ArgumentType::Position3d => argument(name, Position3DArgumentConsumer),
            ArgumentType::Position2d => argument(name, Position2DArgumentConsumer),
            ArgumentType::BlockState => argument(name, BlockArgumentConsumer),
            ArgumentType::BlockPredicate => argument(name, BlockPredicateArgumentConsumer),
            ArgumentType::Item => argument(name, ItemArgumentConsumer),
            ArgumentType::ItemPredicate => argument(name, ItemPredicateArgumentConsumer),
            ArgumentType::Component => argument(name, TextComponentArgConsumer),
            ArgumentType::Rotation => argument(name, RotationArgumentConsumer),
            ArgumentType::ResourceLocation | ArgumentType::Resource(_) => {
                argument(name, ResourceLocationArgumentConsumer)
            }
            ArgumentType::EntityAnchor => argument(name, EntityAnchorArgumentConsumer),
            ArgumentType::Gamemode => argument(name, GamemodeArgumentConsumer),
            ArgumentType::Difficulty => argument(name, DifficultyArgumentConsumer),
            ArgumentType::Time(min) => argument(name, TimeArgumentConsumer::min(min.unwrap_or(0))),
            _ => {
                return Err(wasmtime::Error::msg(format!(
                    "Unimplemented argument type: {arg_type:?}"
                )));
            }
        };
        self.add_command_node(node)
            .map_err(|_| wasmtime::Error::msg("Failed to add argument node"))
    }

    async fn then(
        &mut self,
        self_node: Resource<CommandNode>,
        node: Resource<CommandNode>,
    ) -> wasmtime::Result<()> {
        let child = self.take_node(&node)?;
        let parent = self.get_node_mut(&self_node)?;
        let builder = std::mem::replace(&mut parent.provider, literal(""));
        parent.provider = builder.then(child.provider);
        Ok(())
    }

    async fn execute_with_handler_id(
        &mut self,
        node: Resource<CommandNode>,
        handler_id: u32,
    ) -> wasmtime::Result<()> {
        let plugin = self
            .plugin
            .as_ref()
            .and_then(std::sync::Weak::upgrade)
            .ok_or_else(|| wasmtime::Error::msg("Plugin dropped"))?;
        let server = self
            .server
            .clone()
            .ok_or_else(|| wasmtime::Error::msg("Server not initialized"))?;

        let executor = WasmCommandExecutor {
            handler_id,
            plugin,
            server,
        };
        let resource = self.get_node_mut(&node)?;
        let builder = std::mem::replace(&mut resource.provider, literal(""));
        resource.provider = builder.execute(executor);
        Ok(())
    }

    async fn suggest_with_handler_id(
        &mut self,
        node: Resource<CommandNode>,
        handler_id: u32,
    ) -> wasmtime::Result<()> {
        let plugin = self
            .plugin
            .as_ref()
            .and_then(std::sync::Weak::upgrade)
            .ok_or_else(|| wasmtime::Error::msg("Plugin dropped"))?;
        let server = self
            .server
            .clone()
            .ok_or_else(|| wasmtime::Error::msg("Server not initialized"))?;

        let provider = WasmCommandSuggestionProvider {
            handler_id,
            plugin,
            server,
        };
        let resource = self.get_node_mut(&node)?;
        let builder = std::mem::replace(&mut resource.provider, literal(""));
        resource.provider = builder.suggests(provider);
        Ok(())
    }

    async fn require_with_handler_id(
        &mut self,
        _node: Resource<CommandNode>,
        _handler_id: u32,
    ) -> wasmtime::Result<()> {
        Err(wasmtime::Error::msg(
            "require_with_handler_id not implemented",
        ))
    }

    async fn drop(&mut self, rep: Resource<CommandNode>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<CommandNodeResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}

fn build_bounded_node<T: ToFromNumber + 'static>(
    name: String,
    min: Option<T>,
    max: Option<T>,
) -> NonLeafNodeBuilder
where
    BoundedNumArgumentConsumer<T>: GetClientSideArgParser,
{
    let mut consumer = BoundedNumArgumentConsumer::<T>::new();
    if let Some(m) = min {
        consumer = consumer.min(m);
    }
    if let Some(m) = max {
        consumer = consumer.max(m);
    }

    argument(name, consumer)
}

#[expect(clippy::too_many_lines)]
const fn map_util_locale_to_wit(locale: haskmc_util::translation::Locale) -> Locale {
    match locale {
        haskmc_util::translation::Locale::AfZa => Locale::AfZa,
        haskmc_util::translation::Locale::ArSa => Locale::ArSa,
        haskmc_util::translation::Locale::AstEs => Locale::AstEs,
        haskmc_util::translation::Locale::AzAz => Locale::AzAz,
        haskmc_util::translation::Locale::BaRu => Locale::BaRu,
        haskmc_util::translation::Locale::Bar => Locale::Bar,
        haskmc_util::translation::Locale::BeBy => Locale::BeBy,
        haskmc_util::translation::Locale::BgBg => Locale::BgBg,
        haskmc_util::translation::Locale::BrFr => Locale::BrFr,
        haskmc_util::translation::Locale::Brb => Locale::Brb,
        haskmc_util::translation::Locale::BsBa => Locale::BsBa,
        haskmc_util::translation::Locale::CaEs => Locale::CaEs,
        haskmc_util::translation::Locale::CsCz => Locale::CsCz,
        haskmc_util::translation::Locale::CyGb => Locale::CyGb,
        haskmc_util::translation::Locale::DaDk => Locale::DaDk,
        haskmc_util::translation::Locale::DeAt => Locale::DeAt,
        haskmc_util::translation::Locale::DeCh => Locale::DeCh,
        haskmc_util::translation::Locale::DeDe => Locale::DeDe,
        haskmc_util::translation::Locale::ElGr => Locale::ElGr,
        haskmc_util::translation::Locale::EnAu => Locale::EnAu,
        haskmc_util::translation::Locale::EnCa => Locale::EnCa,
        haskmc_util::translation::Locale::EnGb => Locale::EnGb,
        haskmc_util::translation::Locale::EnNz => Locale::EnNz,
        haskmc_util::translation::Locale::EnPt => Locale::EnPt,
        haskmc_util::translation::Locale::EnUd => Locale::EnUd,
        haskmc_util::translation::Locale::EnUs => Locale::EnUs,
        haskmc_util::translation::Locale::Enp => Locale::Enp,
        haskmc_util::translation::Locale::Enws => Locale::Enws,
        haskmc_util::translation::Locale::EoUy => Locale::EoUy,
        haskmc_util::translation::Locale::EsAr => Locale::EsAr,
        haskmc_util::translation::Locale::EsCl => Locale::EsCl,
        haskmc_util::translation::Locale::EsEc => Locale::EsEc,
        haskmc_util::translation::Locale::EsEs => Locale::EsEs,
        haskmc_util::translation::Locale::EsMx => Locale::EsMx,
        haskmc_util::translation::Locale::EsUy => Locale::EsUy,
        haskmc_util::translation::Locale::EsVe => Locale::EsVe,
        haskmc_util::translation::Locale::Esan => Locale::Esan,
        haskmc_util::translation::Locale::EtEe => Locale::EtEe,
        haskmc_util::translation::Locale::EuEs => Locale::EuEs,
        haskmc_util::translation::Locale::FaIr => Locale::FaIr,
        haskmc_util::translation::Locale::FiFi => Locale::FiFi,
        haskmc_util::translation::Locale::FilPh => Locale::FilPh,
        haskmc_util::translation::Locale::FoFo => Locale::FoFo,
        haskmc_util::translation::Locale::FrCa => Locale::FrCa,
        haskmc_util::translation::Locale::FrFr => Locale::FrFr,
        haskmc_util::translation::Locale::FraDe => Locale::FraDe,
        haskmc_util::translation::Locale::FurIt => Locale::FurIt,
        haskmc_util::translation::Locale::FyNl => Locale::FyNl,
        haskmc_util::translation::Locale::GaIe => Locale::GaIe,
        haskmc_util::translation::Locale::GdGb => Locale::GdGb,
        haskmc_util::translation::Locale::GlEs => Locale::GlEs,
        haskmc_util::translation::Locale::HawUs => Locale::HawUs,
        haskmc_util::translation::Locale::HeIl => Locale::HeIl,
        haskmc_util::translation::Locale::HiIn => Locale::HiIn,
        haskmc_util::translation::Locale::HrHr => Locale::HrHr,
        haskmc_util::translation::Locale::HuHu => Locale::HuHu,
        haskmc_util::translation::Locale::HyAm => Locale::HyAm,
        haskmc_util::translation::Locale::IdId => Locale::IdId,
        haskmc_util::translation::Locale::IgNg => Locale::IgNg,
        haskmc_util::translation::Locale::IoEn => Locale::IoEn,
        haskmc_util::translation::Locale::IsIs => Locale::IsIs,
        haskmc_util::translation::Locale::Isv => Locale::Isv,
        haskmc_util::translation::Locale::ItIt => Locale::ItIt,
        haskmc_util::translation::Locale::JaJp => Locale::JaJp,
        haskmc_util::translation::Locale::JboEn => Locale::JboEn,
        haskmc_util::translation::Locale::KaGe => Locale::KaGe,
        haskmc_util::translation::Locale::KkKz => Locale::KkKz,
        haskmc_util::translation::Locale::KnIn => Locale::KnIn,
        haskmc_util::translation::Locale::KoKr => Locale::KoKr,
        haskmc_util::translation::Locale::Ksh => Locale::Ksh,
        haskmc_util::translation::Locale::KwGb => Locale::KwGb,
        haskmc_util::translation::Locale::LaLa => Locale::LaLa,
        haskmc_util::translation::Locale::LbLu => Locale::LbLu,
        haskmc_util::translation::Locale::LiLi => Locale::LiLi,
        haskmc_util::translation::Locale::Lmo => Locale::Lmo,
        haskmc_util::translation::Locale::LoLa => Locale::LoLa,
        haskmc_util::translation::Locale::LolUs => Locale::LolUs,
        haskmc_util::translation::Locale::LtLt => Locale::LtLt,
        haskmc_util::translation::Locale::LvLv => Locale::LvLv,
        haskmc_util::translation::Locale::Lzh => Locale::Lzh,
        haskmc_util::translation::Locale::MkMk => Locale::MkMk,
        haskmc_util::translation::Locale::MnMn => Locale::MnMn,
        haskmc_util::translation::Locale::MsMy => Locale::MsMy,
        haskmc_util::translation::Locale::MtMt => Locale::MtMt,
        haskmc_util::translation::Locale::Nah => Locale::Nah,
        haskmc_util::translation::Locale::NdsDe => Locale::NdsDe,
        haskmc_util::translation::Locale::NlBe => Locale::NlBe,
        haskmc_util::translation::Locale::NlNl => Locale::NlNl,
        haskmc_util::translation::Locale::NnNo => Locale::NnNo,
        haskmc_util::translation::Locale::NoNo => Locale::NoNo,
        haskmc_util::translation::Locale::OcFr => Locale::OcFr,
        haskmc_util::translation::Locale::Ovd => Locale::Ovd,
        haskmc_util::translation::Locale::PlPl => Locale::PlPl,
        haskmc_util::translation::Locale::PtBr => Locale::PtBr,
        haskmc_util::translation::Locale::PtPt => Locale::PtPt,
        haskmc_util::translation::Locale::QyaAa => Locale::QyaAa,
        haskmc_util::translation::Locale::RoRo => Locale::RoRo,
        haskmc_util::translation::Locale::Rpr => Locale::Rpr,
        haskmc_util::translation::Locale::RuRu => Locale::RuRu,
        haskmc_util::translation::Locale::RyUa => Locale::RyUa,
        haskmc_util::translation::Locale::SahSah => Locale::SahSah,
        haskmc_util::translation::Locale::SeNo => Locale::SeNo,
        haskmc_util::translation::Locale::SkSk => Locale::SkSk,
        haskmc_util::translation::Locale::SlSi => Locale::SlSi,
        haskmc_util::translation::Locale::SoSo => Locale::SoSo,
        haskmc_util::translation::Locale::SqAl => Locale::SqAl,
        haskmc_util::translation::Locale::SrCs => Locale::SrCs,
        haskmc_util::translation::Locale::SrSp => Locale::SrSp,
        haskmc_util::translation::Locale::SvSe => Locale::SvSe,
        haskmc_util::translation::Locale::Sxu => Locale::Sxu,
        haskmc_util::translation::Locale::Szl => Locale::Szl,
        haskmc_util::translation::Locale::TaIn => Locale::TaIn,
        haskmc_util::translation::Locale::ThTh => Locale::ThTh,
        haskmc_util::translation::Locale::TlPh => Locale::TlPh,
        haskmc_util::translation::Locale::TlhAa => Locale::TlhAa,
        haskmc_util::translation::Locale::Tok => Locale::Tok,
        haskmc_util::translation::Locale::TrTr => Locale::TrTr,
        haskmc_util::translation::Locale::TtRu => Locale::TtRu,
        haskmc_util::translation::Locale::UkUa => Locale::UkUa,
        haskmc_util::translation::Locale::ValEs => Locale::ValEs,
        haskmc_util::translation::Locale::VecIt => Locale::VecIt,
        haskmc_util::translation::Locale::ViVn => Locale::ViVn,
        haskmc_util::translation::Locale::YiDe => Locale::YiDe,
        haskmc_util::translation::Locale::YoNg => Locale::YoNg,
        haskmc_util::translation::Locale::ZhCn => Locale::ZhCn,
        haskmc_util::translation::Locale::ZhHk => Locale::ZhHk,
        haskmc_util::translation::Locale::ZhTw => Locale::ZhTw,
        haskmc_util::translation::Locale::ZlmArab => Locale::ZlmArab,
    }
}
