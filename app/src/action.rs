pub enum Action {
    /// Turns off the renderer of the object with the specified id
    EnableSingleRenderer(u64),

    // ----- GAME -----
    /// Signals that the current question gives up its place and a new one should be picked
    SwitchToNextQuestion,
    /// Signals that the current question should run the generic_to_next_part() function
    /// The question may then switch to its next part, or give up control
    SwitchToNextPart,
}
