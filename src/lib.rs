use action::action_state_system;
use actor::{actor_state_system, build_new_actor_system};
use bevy::prelude::{
    First, IntoSystemConfigs, IntoSystemSetConfigs, Last, Plugin, PostUpdate, Startup, SystemSet,
};

use planning::{
    create_plan_system, create_planning_state, request_plan_event_handler_system, RequestPlanEvent,
};

mod action;
mod actor;
mod common;
mod condition;
mod planning;
mod state;

#[cfg(feature = "inspector")]
pub mod inspector;

pub use action::{Action, ActionState, EvaluationResult};
pub use actor::{Actor, ActorState};
pub use condition::Condition;

pub struct GoapPlugin;

impl Plugin for GoapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<RequestPlanEvent>();

        app.add_systems(Startup, create_planning_state);

        app.add_systems(First, build_new_actor_system);

        // User Action systems should be added to GoapSet::Actions, which can check for progress of Actions after typical user systems (e.g. for movement) complete during Update.
        // InternalGoapSet::ActionStateTransition runs after, for change detection of completed or failed actions, which may update the ActorState to reflect a completed or failed plan.
        // User Actor systems should be added to GoapSet::Actors, which can react to an Actor's completed or failed plan.
        // We add InternalGoapSet::ActorStateTransition for change detection of ActorStates for Actors that may require a new plan.
        app.configure_sets(
            PostUpdate,
            (GoapSet::Actions, InternalGoapSet::ActionStateTransition, GoapSet::Actors, InternalGoapSet::ActorStateTransition).chain(),
        )
        .add_systems(PostUpdate, action_state_system.in_set(InternalGoapSet::ActionStateTransition))
        .add_systems(
            PostUpdate,
            (
                actor_state_system,
                request_plan_event_handler_system.after(actor_state_system),
            )
                .in_set(InternalGoapSet::ActorStateTransition),
        );

        app.add_systems(Last, create_plan_system);
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, SystemSet)]
pub enum GoapSet {
    /// User `Action` systems should be added to this system set.
    Actions,
    /// User `Actor` systems should be added to this system set.
    Actors,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, SystemSet)]
enum InternalGoapSet {
    /// Internal system set to react to changed `ActionState`s from user `Action` systems.
    ActionStateTransition,
    /// Internal system set to react to changed `ActorState`s from user `Actor` systems.
    ActorStateTransition,
}
