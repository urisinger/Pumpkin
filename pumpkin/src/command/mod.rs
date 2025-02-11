use std::fmt;
use std::sync::Arc;

use crate::command::commands::seed;
use crate::command::commands::{bossbar, transfer};
use crate::command::dispatcher::CommandDispatcher;
use crate::entity::player::Player;
use crate::server::Server;
use crate::world::World;
use args::ConsumedArgs;
use async_trait::async_trait;
use commands::{
    ban, banip, banlist, clear, damage, deop, experience, fill, gamemode, give, help, kick, kill,
    list, me, msg, op, pardon, pardonip, particle, playsound, plugin, plugins, pumpkin, say,
    setblock, stop, summon, teleport, time, title, weather, worldborder,
};
use dispatcher::CommandError;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_util::permission::PermissionLvl;
use pumpkin_util::text::TextComponent;

pub mod args;
pub mod client_suggestions;
mod commands;
pub mod dispatcher;
pub mod tree;

pub enum CommandSender<'a> {
    Rcon(&'a tokio::sync::Mutex<Vec<String>>),
    Console,
    Player(Arc<Player>),
}

impl fmt::Display for CommandSender<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommandSender::Console => "Server",
                CommandSender::Rcon(_) => "Rcon",
                CommandSender::Player(p) => &p.gameprofile.name,
            }
        )
    }
}

impl CommandSender<'_> {
    pub async fn send_message(&self, text: TextComponent) {
        match self {
            CommandSender::Console => log::info!("{}", text.to_pretty_console()),
            CommandSender::Player(c) => c.send_system_message(&text).await,
            CommandSender::Rcon(s) => s.lock().await.push(text.to_pretty_console()),
        }
    }

    #[must_use]
    pub const fn is_player(&self) -> bool {
        matches!(self, CommandSender::Player(_))
    }

    #[must_use]
    pub const fn is_console(&self) -> bool {
        matches!(self, CommandSender::Console)
    }
    #[must_use]
    pub fn as_player(&self) -> Option<Arc<Player>> {
        match self {
            CommandSender::Player(player) => Some(player.clone()),
            _ => None,
        }
    }

    /// prefer using `has_permission_lvl(lvl)`
    #[must_use]
    pub fn permission_lvl(&self) -> PermissionLvl {
        match self {
            CommandSender::Console | CommandSender::Rcon(_) => PermissionLvl::Four,
            CommandSender::Player(p) => p.permission_lvl.load(),
        }
    }

    #[must_use]
    pub fn has_permission_lvl(&self, lvl: PermissionLvl) -> bool {
        match self {
            CommandSender::Console | CommandSender::Rcon(_) => true,
            CommandSender::Player(p) => p.permission_lvl.load().ge(&lvl),
        }
    }

    #[must_use]
    pub fn position(&self) -> Option<Vector3<f64>> {
        match self {
            CommandSender::Console | CommandSender::Rcon(..) => None,
            CommandSender::Player(p) => Some(p.living_entity.entity.pos.load()),
        }
    }

    #[must_use]
    pub async fn world(&self) -> Option<Arc<World>> {
        match self {
            // TODO: maybe return first world when console
            CommandSender::Console | CommandSender::Rcon(..) => None,
            CommandSender::Player(p) => Some(p.living_entity.entity.world.read().await.clone()),
        }
    }
}

#[must_use]
pub fn default_dispatcher() -> CommandDispatcher {
    let mut dispatcher = CommandDispatcher::default();

    // Zerp
    dispatcher.register(pumpkin::init_command_tree(), PermissionLvl::Zero);
    dispatcher.register(help::init_command_tree(), PermissionLvl::Zero);
    dispatcher.register(list::init_command_tree(), PermissionLvl::Zero);
    dispatcher.register(transfer::init_command_tree(), PermissionLvl::Zero);
    dispatcher.register(me::init_command_tree(), PermissionLvl::Zero);
    dispatcher.register(msg::init_command_tree(), PermissionLvl::Zero);
    // Two
    dispatcher.register(kill::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(worldborder::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(teleport::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(time::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(give::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(clear::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(setblock::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(seed::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(fill::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(playsound::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(title::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(summon::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(experience::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(weather::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(particle::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(damage::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(bossbar::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(say::init_command_tree(), PermissionLvl::Two);
    dispatcher.register(gamemode::init_command_tree(), PermissionLvl::Two);
    // Three
    dispatcher.register(op::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(deop::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(kick::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(plugin::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(plugins::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(ban::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(banip::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(banlist::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(pardon::init_command_tree(), PermissionLvl::Three);
    dispatcher.register(pardonip::init_command_tree(), PermissionLvl::Three);
    // Four
    dispatcher.register(stop::init_command_tree(), PermissionLvl::Four);

    dispatcher
}

#[async_trait]
pub trait CommandExecutor: Sync {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError>;
}
