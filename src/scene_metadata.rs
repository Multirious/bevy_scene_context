use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Added, Changed, QueryEntityError, With, Without},
    removal_detection::RemovedComponents,
    system::{Commands, Query},
};
use bevy_hierarchy::Parent;
use bevy_scene::SceneInstance;

struct SceneMetadataPlugin;

impl Plugin for SceneMetadataPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(PreUpdate, scene_metadata_system);
    }
}

#[derive(Component)]
pub struct SceneMetadata {
    root: Entity,
}

#[allow(clippy::type_complexity)]
pub fn scene_metadata_system(
    commands: Commands,
    new_scene: Query<Entity, Added<SceneInstance>>,
    changed_parent: Query<(&Parent, &SceneMetadata), (Changed<Parent>, Without<SceneInstance>)>,
    removed_parent: RemovedComponents<Parent>,
) {
    new_scene.iter().for_each(|entity| {
        // commands.entity()
    });
    changed_parent.iter().for_each(|(parent, metadata)| {})
}

fn search_scene_root(
    entity: Entity,
    q_parent: Query<(&Parent, Option<&SceneInstance>)>,
) -> Result<Entity, QueryEntityError> {
    // q_scene_root.get(entity)
    todo!()
}
