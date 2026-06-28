use eframe::emath;
use nalgebra::Vector3;

use crate::{
    enums::{GameStage, RendererCategory},
    sky::markers::game_markers,
};

pub enum Action {
    /// The user clicked the screen on this position
    ScreenClicked(emath::Pos2),
    /// The user dragged between the two provided positions
    ScreenDragged(ScreenDraggedData),

    /// Directs the camera to look in the direction of the specified 3D point (the 3D point comes from a frame fixed to the RA-DEC coordinates)
    CameraLookAt(Vector3<f32>),
    /// Zooms the camera using the provided velocity
    CameraZoom(f32),
    /// Turns on the renderer of the object with the specified id
    DisableSingleRenderer(u64),
    /// Turns off the renderer of the object with the specified id
    EnableSingleRenderer(u64),
    /// Reinitialises the renderer group with the category and name provided
    InitSingleRendererGroup(RendererCategory, String),

    /// Sets whether or not markers should be added when the screen is clicked
    SetAddMarkerOnClick(bool),
    /// Sets whether or not input should be requested for an input field; used in questions using text fields for answers, where input is requested for the text field on the first frame of rendering the question
    SetRequestInputFocus(bool),
    /// Sets whether or not the question window is open
    ToggleQuestionWindow(bool),

    // ----- GAME -----
    /// Signals that the current question gives up its place and a new one should be picked
    SwitchToNextQuestion,
    /// Signals that the current question should run the generic_to_next_part() function
    /// The question may then switch to its next part, or give up control
    SwitchToNextPart,
    /// Adds a game marker
    AddGameMarker(game_markers::GameMarker),
    /// Sets the game markers to the provided list
    SetGameMarkers(Vec<game_markers::GameMarker>),
    /// Removes all game markers
    RemoveGameMarkers,
    /// Sets the score to the specified amount
    SetScore(u32),
    /// Sets the maximum possible score tp the specified amount
    SetPossibleScore(u32),
    /// Changes the score by the specified amount
    ChangeScore(u32),
    /// Changes the maximum possible score by the specified amount
    ChangePossibleScore(u32),
    /// Sets the game stage
    SetGameStage(GameStage),
    /// Marks the question with the provided index as used so that it is not asked again until the question deck is reset
    MarkQuestionAsUsed(usize),
    /// Increments the counter keeping track of how many questions were played without blocking them from appearing again (usually by answering incorrectly)
    IncrementRepeatedQuestionCounter,
}

pub struct ScreenDraggedData {
    pub from: emath::Pos2,
    pub to: emath::Pos2,
}

impl ScreenDraggedData {
    pub fn new(from: emath::Pos2, to: emath::Pos2) -> Self {
        Self { from, to }
    }
}
