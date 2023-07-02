pub fn is_in_rect<T: PartialOrd>(point: [T; 2], rect: [[T; 2]; 2]) -> bool {
	let [upper_left, bottom_right] = rect;
	point[0] >= upper_left[0] && point[0] <= bottom_right[0] && point[1] >= upper_left[1] && point[1] <= bottom_right[1]
}