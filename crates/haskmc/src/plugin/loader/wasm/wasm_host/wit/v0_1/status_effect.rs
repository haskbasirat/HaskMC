use crate::plugin::loader::wasm::wasm_host::{
    state::PluginHostState, wit::v0_1::pumpkin::plugin::status_effect,
};

impl status_effect::Host for PluginHostState {}

#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn from_wasm_status_effect_type(
    t: status_effect::StatusEffectType,
) -> haskmc_data::status_effect::EffectType {
    match t {
        status_effect::StatusEffectType::Speed => haskmc_data::status_effect::EffectType::Speed,
        status_effect::StatusEffectType::Slowness => {
            haskmc_data::status_effect::EffectType::Slowness
        }
        status_effect::StatusEffectType::Haste => haskmc_data::status_effect::EffectType::Haste,
        status_effect::StatusEffectType::MiningFatigue => {
            haskmc_data::status_effect::EffectType::MiningFatigue
        }
        status_effect::StatusEffectType::Strength => {
            haskmc_data::status_effect::EffectType::Strength
        }
        status_effect::StatusEffectType::InstantHealth => {
            haskmc_data::status_effect::EffectType::InstantHealth
        }
        status_effect::StatusEffectType::InstantDamage => {
            haskmc_data::status_effect::EffectType::InstantDamage
        }
        status_effect::StatusEffectType::JumpBoost => {
            haskmc_data::status_effect::EffectType::JumpBoost
        }
        status_effect::StatusEffectType::Nausea => haskmc_data::status_effect::EffectType::Nausea,
        status_effect::StatusEffectType::Regeneration => {
            haskmc_data::status_effect::EffectType::Regeneration
        }
        status_effect::StatusEffectType::Resistance => {
            haskmc_data::status_effect::EffectType::Resistance
        }
        status_effect::StatusEffectType::FireResistance => {
            haskmc_data::status_effect::EffectType::FireResistance
        }
        status_effect::StatusEffectType::WaterBreathing => {
            haskmc_data::status_effect::EffectType::WaterBreathing
        }
        status_effect::StatusEffectType::Invisibility => {
            haskmc_data::status_effect::EffectType::Invisibility
        }
        status_effect::StatusEffectType::Blindness => {
            haskmc_data::status_effect::EffectType::Blindness
        }
        status_effect::StatusEffectType::NightVision => {
            haskmc_data::status_effect::EffectType::NightVision
        }
        status_effect::StatusEffectType::Hunger => haskmc_data::status_effect::EffectType::Hunger,
        status_effect::StatusEffectType::Weakness => {
            haskmc_data::status_effect::EffectType::Weakness
        }
        status_effect::StatusEffectType::Poison => haskmc_data::status_effect::EffectType::Poison,
        status_effect::StatusEffectType::Wither => haskmc_data::status_effect::EffectType::Wither,
        status_effect::StatusEffectType::HealthBoost => {
            haskmc_data::status_effect::EffectType::HealthBoost
        }
        status_effect::StatusEffectType::Absorption => {
            haskmc_data::status_effect::EffectType::Absorption
        }
        status_effect::StatusEffectType::Saturation => {
            haskmc_data::status_effect::EffectType::Saturation
        }
        status_effect::StatusEffectType::Glowing => haskmc_data::status_effect::EffectType::Glowing,
        status_effect::StatusEffectType::Levitation => {
            haskmc_data::status_effect::EffectType::Levitation
        }
        status_effect::StatusEffectType::Luck => haskmc_data::status_effect::EffectType::Luck,
        status_effect::StatusEffectType::Unluck => haskmc_data::status_effect::EffectType::Unluck,
        status_effect::StatusEffectType::SlowFalling => {
            haskmc_data::status_effect::EffectType::SlowFalling
        }
        status_effect::StatusEffectType::ConduitPower => {
            haskmc_data::status_effect::EffectType::ConduitPower
        }
        status_effect::StatusEffectType::DolphinsGrace => {
            haskmc_data::status_effect::EffectType::DolphinsGrace
        }
        status_effect::StatusEffectType::BadOmen => haskmc_data::status_effect::EffectType::BadOmen,
        status_effect::StatusEffectType::HeroOfTheVillage => {
            haskmc_data::status_effect::EffectType::HeroOfTheVillage
        }
        status_effect::StatusEffectType::Darkness => {
            haskmc_data::status_effect::EffectType::Darkness
        }
        status_effect::StatusEffectType::TrialOmen => {
            haskmc_data::status_effect::EffectType::TrialOmen
        }
        status_effect::StatusEffectType::RaidOmen => {
            haskmc_data::status_effect::EffectType::RaidOmen
        }
        status_effect::StatusEffectType::WindCharged => {
            haskmc_data::status_effect::EffectType::WindCharged
        }
        status_effect::StatusEffectType::Weaving => haskmc_data::status_effect::EffectType::Weaving,
        status_effect::StatusEffectType::Oozing => haskmc_data::status_effect::EffectType::Oozing,
        status_effect::StatusEffectType::Infested => {
            haskmc_data::status_effect::EffectType::Infested
        }
    }
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn to_wasm_status_effect_type(
    t: haskmc_data::status_effect::EffectType,
) -> status_effect::StatusEffectType {
    match t {
        haskmc_data::status_effect::EffectType::Speed => status_effect::StatusEffectType::Speed,
        haskmc_data::status_effect::EffectType::Slowness => {
            status_effect::StatusEffectType::Slowness
        }
        haskmc_data::status_effect::EffectType::Haste => status_effect::StatusEffectType::Haste,
        haskmc_data::status_effect::EffectType::MiningFatigue => {
            status_effect::StatusEffectType::MiningFatigue
        }
        haskmc_data::status_effect::EffectType::Strength => {
            status_effect::StatusEffectType::Strength
        }
        haskmc_data::status_effect::EffectType::InstantHealth => {
            status_effect::StatusEffectType::InstantHealth
        }
        haskmc_data::status_effect::EffectType::InstantDamage => {
            status_effect::StatusEffectType::InstantDamage
        }
        haskmc_data::status_effect::EffectType::JumpBoost => {
            status_effect::StatusEffectType::JumpBoost
        }
        haskmc_data::status_effect::EffectType::Nausea => status_effect::StatusEffectType::Nausea,
        haskmc_data::status_effect::EffectType::Regeneration => {
            status_effect::StatusEffectType::Regeneration
        }
        haskmc_data::status_effect::EffectType::Resistance => {
            status_effect::StatusEffectType::Resistance
        }
        haskmc_data::status_effect::EffectType::FireResistance => {
            status_effect::StatusEffectType::FireResistance
        }
        haskmc_data::status_effect::EffectType::WaterBreathing => {
            status_effect::StatusEffectType::WaterBreathing
        }
        haskmc_data::status_effect::EffectType::Invisibility => {
            status_effect::StatusEffectType::Invisibility
        }
        haskmc_data::status_effect::EffectType::Blindness => {
            status_effect::StatusEffectType::Blindness
        }
        haskmc_data::status_effect::EffectType::NightVision => {
            status_effect::StatusEffectType::NightVision
        }
        haskmc_data::status_effect::EffectType::Hunger => status_effect::StatusEffectType::Hunger,
        haskmc_data::status_effect::EffectType::Weakness => {
            status_effect::StatusEffectType::Weakness
        }
        haskmc_data::status_effect::EffectType::Poison => status_effect::StatusEffectType::Poison,
        haskmc_data::status_effect::EffectType::Wither => status_effect::StatusEffectType::Wither,
        haskmc_data::status_effect::EffectType::HealthBoost => {
            status_effect::StatusEffectType::HealthBoost
        }
        haskmc_data::status_effect::EffectType::Absorption => {
            status_effect::StatusEffectType::Absorption
        }
        haskmc_data::status_effect::EffectType::Saturation => {
            status_effect::StatusEffectType::Saturation
        }
        haskmc_data::status_effect::EffectType::Glowing => status_effect::StatusEffectType::Glowing,
        haskmc_data::status_effect::EffectType::Levitation => {
            status_effect::StatusEffectType::Levitation
        }
        haskmc_data::status_effect::EffectType::Luck => status_effect::StatusEffectType::Luck,
        haskmc_data::status_effect::EffectType::Unluck => status_effect::StatusEffectType::Unluck,
        haskmc_data::status_effect::EffectType::SlowFalling => {
            status_effect::StatusEffectType::SlowFalling
        }
        haskmc_data::status_effect::EffectType::ConduitPower => {
            status_effect::StatusEffectType::ConduitPower
        }
        haskmc_data::status_effect::EffectType::DolphinsGrace => {
            status_effect::StatusEffectType::DolphinsGrace
        }
        haskmc_data::status_effect::EffectType::BadOmen => status_effect::StatusEffectType::BadOmen,
        haskmc_data::status_effect::EffectType::HeroOfTheVillage => {
            status_effect::StatusEffectType::HeroOfTheVillage
        }
        haskmc_data::status_effect::EffectType::Darkness => {
            status_effect::StatusEffectType::Darkness
        }
        haskmc_data::status_effect::EffectType::TrialOmen => {
            status_effect::StatusEffectType::TrialOmen
        }
        haskmc_data::status_effect::EffectType::RaidOmen => {
            status_effect::StatusEffectType::RaidOmen
        }
        haskmc_data::status_effect::EffectType::WindCharged => {
            status_effect::StatusEffectType::WindCharged
        }
        haskmc_data::status_effect::EffectType::Weaving => status_effect::StatusEffectType::Weaving,
        haskmc_data::status_effect::EffectType::Oozing => status_effect::StatusEffectType::Oozing,
        haskmc_data::status_effect::EffectType::Infested => {
            status_effect::StatusEffectType::Infested
        }
    }
}

#[must_use]
pub fn to_wasm_status_effect_instance(
    effect: &haskmc_data::potion::Effect,
) -> Option<status_effect::StatusEffectInstance> {
    let name = effect
        .effect_type
        .minecraft_name
        .strip_prefix("minecraft:")
        .unwrap_or(effect.effect_type.minecraft_name);
    let effect_type_enum = haskmc_data::status_effect::EffectType::from_name(name)?;
    let wasm_type = to_wasm_status_effect_type(effect_type_enum);
    Some(status_effect::StatusEffectInstance {
        effect_type: wasm_type,
        duration: u32::try_from(effect.duration).unwrap_or(0),
        amplifier: effect.amplifier,
        ambient: effect.ambient,
        show_particles: effect.show_particles,
        show_icon: effect.show_icon,
    })
}
