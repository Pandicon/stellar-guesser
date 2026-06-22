use nalgebra::Vector3;

pub enum Action {
    /// Directs the camera to look in the direction of the specified 3D point (the 3D point comes from a frame fixed to the RA-DEC coordinates)
    CameraLookAt(Vector3<f32>),
    /// Turns on the renderer of the object with the specified id
    DisableSingleRenderer(u64),
    /// Turns off the renderer of the object with the specified id
    EnableSingleRenderer(u64),

    // ----- GAME -----
    /// Signals that the current question gives up its place and a new one should be picked
    SwitchToNextQuestion,
    /// Signals that the current question should run the generic_to_next_part() function
    /// The question may then switch to its next part, or give up control
    SwitchToNextPart,
}
