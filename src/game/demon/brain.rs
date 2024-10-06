use crate::prelude::*;

use behavior_bark::unpowered::*;
use serde::{Deserialize, Serialize};

use super::{pick_random_tool, roll_characteristic, tool_time, DemonDna};

pub struct DemonModel {
    pub nearest_tool: DeskItem,
    pub in_range_of_tool: bool,
    pub using_tool: Option<f32>,
    pub dna: DemonDna,
    pub nonce: u32, // Updates each time the tree is run fully.
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DemonController {
    Idle,
    MoveTo(DeskItem),
    UseTool,
    FinishJob,
    Distracted(Distraction, Box<DemonController>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DemonBehavior {
    NotDistracted,
    CheckDistraction(Distraction),
    Distraction(Distraction),
    DoILikeNearestTool,
    MoveToNearestTool,
    MoveToRandomTool,
    UseTool,
}

impl UnpoweredFunction for DemonBehavior {
    type Model = DemonModel;
    type Controller = DemonController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            DemonBehavior::NotDistracted => {
                if matches!(controller, DemonController::Distracted(_, _)) {
                    UnpoweredFunctionState::Waiting
                } else {
                    UnpoweredFunctionState::Complete
                }
            }
            DemonBehavior::CheckDistraction(distraction) => {
                if roll_characteristic(
                    &model.dna,
                    distraction.gene_idx(),
                    model.nonce,
                    distraction.chance_basis(),
                ) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            DemonBehavior::Distraction(distraction) => {
                *controller =
                    DemonController::Distracted(distraction.clone(), Box::new(controller.clone()));
                UnpoweredFunctionState::Complete
            }
            DemonBehavior::DoILikeNearestTool => {
                if model.nearest_tool == DeskItem::Summoning && model.nonce < 10 {
                    return UnpoweredFunctionState::Failed;
                }
                let like_tool = roll_characteristic(&model.dna, 3, model.nonce, 0.5);
                if like_tool {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            DemonBehavior::UseTool => {
                if model.in_range_of_tool {
                    if let Some(time) = model.using_tool {
                        let target_time = tool_time(&model.dna, model.nonce, 2., 8.);
                        if time < target_time {
                            println!("Using tool for {:.2}s", time);
                            *controller = DemonController::UseTool;
                            UnpoweredFunctionState::Waiting
                        } else {
                            println!("Finished using tool");
                            *controller = DemonController::FinishJob;
                            UnpoweredFunctionState::Complete
                        }
                    } else {
                        println!("Using tool");
                        *controller = DemonController::UseTool;
                        UnpoweredFunctionState::Waiting
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            DemonBehavior::MoveToNearestTool => {
                *controller = DemonController::MoveTo(model.nearest_tool);
                UnpoweredFunctionState::Complete
            }
            DemonBehavior::MoveToRandomTool => {
                let tool = pick_random_tool(&model.dna, model.nonce);
                if model.in_range_of_tool && model.nearest_tool == tool {
                    UnpoweredFunctionState::Complete
                } else {
                    *controller = DemonController::MoveTo(tool);
                    UnpoweredFunctionState::Waiting
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Distraction {
    Berate,
    Complain,
    Sleep,
    Annoyed,
    Wander,
}

impl Distraction {
    pub fn gene_idx(&self) -> usize {
        match self {
            Distraction::Complain => 0,
            Distraction::Sleep => 1,
            Distraction::Annoyed => 2,
            Distraction::Wander => 3,
            Distraction::Berate => unreachable!(),
        }
    }

    pub fn chance_basis(&self) -> f32 {
        match self {
            Distraction::Complain => 0.5,
            Distraction::Sleep => 0.1,
            Distraction::Annoyed => 0.9,
            Distraction::Wander => 0.9,
            Distraction::Berate => unreachable!(),
        }
    }
}

pub type DemonBrainNode =
    dyn UnpoweredFunction<Model = DemonModel, Controller = DemonController> + Sync + Send;
pub type DemonBrainDef = UnpoweredTreeDef<DemonBehavior, ()>;

#[derive(Component)]
pub struct DemonBrain(pub Box<DemonBrainNode>);
