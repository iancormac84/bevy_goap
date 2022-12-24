use bevy::prelude::*;
use bevy_goap::{Action, ActionState, Actor, Condition, GoapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GoapPlugin)
        .add_startup_system(create_lumberjack)
        .add_startup_system(create_lumberjack)
        .add_startup_system(create_axes_system)
        .add_system(get_axe_action_system)
        .add_system(chop_tree_action_system)
        .add_system(collect_wood_action_system)
        .run();
}

fn create_lumberjack(mut commands: Commands) {
    let get_axe_action = Action::build(GetAxeAction)
        .with_precondition(ActorHasAxeCondition, false)
        .with_postcondition(ActorHasAxeCondition, true);

    let chop_tree_action = Action::build(ChopTreeAction::new(5))
        .with_precondition(ActorHasAxeCondition, true)
        .with_postcondition(ActorHasWoodCondition, true);

    let _collect_wood_action = Action::build(CollectWoodAction)
        .with_precondition(ActorHasWoodCondition, false)
        .with_postcondition(ActorHasWoodCondition, true);

    // Try toggling the initial conditions and removing actions to observe the different action sequences the lumberjack performs!
    let lumberjack = Actor::build(Lumberjack)
        .with_initial_condition(ActorHasAxeCondition, false)
        .with_initial_condition(ActorHasWoodCondition, false)
        .with_goal(ActorHasWoodCondition, true)
        .with_action(get_axe_action)
        // .with_action(_collect_wood_action) // Try uncommenting this action to observe a different action sequence the lumberjack performs!
        .with_action(chop_tree_action);

    commands.spawn_empty().insert(lumberjack);
}

#[derive(Component, Clone)]
struct Lumberjack;

#[derive(Component, Clone)]
struct GetAxeAction;

fn get_axe_action_system(
    mut query: Query<(&Action, &mut ActionState), With<GetAxeAction>>,
    mut axes: Query<&mut Axe>,
) {
    let mut unclaimed_axes = axes
        .iter_mut()
        .filter(|axe| axe.owner.is_none())
        .collect::<Vec<_>>();

    for (action, mut action_state) in query.iter_mut() {
        match *action_state {
            ActionState::Evaluate => {
                println!("Evaluating GetAxeAction");
                if let Some(mut axe) = unclaimed_axes.pop() {
                    println!("Claimed an axe!");

                    axe.owner = Some(action.actor_entity);

                    *action_state = ActionState::EvaluationSuccess;
                } else {
                    println!("No available axe to claim!");

                    *action_state = ActionState::EvaluationFailure;
                }
            }
            ActionState::Started => {
                println!("Starting GetAxeAction");

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                println!("Getting axe!");

                *action_state = ActionState::Complete;
            }
            ActionState::Complete => {
                println!("GetAxeAction is Complete.");
            }
            _ => {}
        };
    }
}

struct ActorHasAxeCondition;
impl Condition for ActorHasAxeCondition {}

#[derive(Component, Clone)]
struct ChopTreeAction {
    max_chops: u8,
    current_chops: u8,
}

impl ChopTreeAction {
    fn new(max_chops: u8) -> Self {
        Self {
            max_chops,
            current_chops: 0,
        }
    }
}

fn chop_tree_action_system(mut query: Query<(&mut ActionState, &mut ChopTreeAction)>) {
    for (mut action_state, mut chop_tree_action) in query.iter_mut() {
        match *action_state {
            ActionState::Evaluate => {
                *action_state = ActionState::EvaluationSuccess;
            }
            ActionState::Started => {
                println!("Starting to chop!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                chop_tree_action.current_chops += 1;
                println!("Chopped tree {} times!", chop_tree_action.current_chops);

                if chop_tree_action.current_chops >= chop_tree_action.max_chops {
                    *action_state = ActionState::Complete;
                }
            }
            _ => {}
        }
    }
}

#[derive(Component, Clone)]
struct CollectWoodAction;

fn collect_wood_action_system(mut query: Query<&mut ActionState, With<CollectWoodAction>>) {
    for mut action_state in query.iter_mut() {
        match *action_state {
            ActionState::Started => {
                println!("Starting to collect wood!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                println!("Collecting wood!");
                *action_state = ActionState::Complete;
            }
            _ => (),
        }
    }
}

struct ActorHasWoodCondition;
impl Condition for ActorHasWoodCondition {}

#[derive(Component)]
struct Axe {
    owner: Option<Entity>,
}

fn create_axes_system(mut commands: Commands) {
    commands.spawn_empty().insert(Axe { owner: None });
}
