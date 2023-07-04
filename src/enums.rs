use eframe::epaint::Pos2;

pub enum Inputs {
	AltShiftI,
	AltShiftS,
}

pub enum PointerPosition {
	OnScreen(Pos2),
	OffScreen
	
}
