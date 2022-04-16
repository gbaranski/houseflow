use std::collections::HashSet;

use acu::MasterExt;
use futures::future::join_all;
use houseflow_types::{
    accessory::{self, Accessory},
    permission::Permission,
    user,
};

use crate::{
    extensions::Config,
    providers::{self, ProviderExt},
};

pub async fn get_user_accessories(
    config: &Config,
    master_provider: &providers::MasterHandle,
    user_id: &user::ID,
) -> Vec<Accessory> {
    let user_structures = {
        let config = config.get();
        let structures = config.get_user_structures(&user_id);
        structures
            .into_iter()
            .map(|structure| structure.id)
            .collect::<HashSet<_>>()
    };
    let slaves = master_provider.slaves().await;
    let futures = slaves.iter().map(|slave| slave.get_accessories());
    let accessories = join_all(futures).await;
    accessories
        .iter()
        .flatten()
        .filter(|(structure, _)| user_structures.contains(structure))
        .map(|(_, accessory)| accessory)
        .flatten()
        .cloned()
        .collect()
}

pub async fn get_permission(
    config: &Config,
    master_provider: &providers::MasterHandle,
    user_id: &user::ID,
    accessory_id: &accessory::ID,
) -> Option<Permission> {
    let slaves = master_provider.slaves().await;
    let futures = slaves.iter().map(|slave| slave.get_accessories());
    let accessories = join_all(futures).await;
    let permission = accessories
        .iter()
        .flatten()
        .find(|(_, accessories)| {
            accessories
                .iter()
                .any(|accessory| accessory.id == *accessory_id)
        })
        .map(|(structure, _)| {
            let config = config.get();
            config.get_permission(structure, user_id).cloned().unwrap()
        });
    permission
}
