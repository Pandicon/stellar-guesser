use super::{LineSegment, Rectangle};

pub fn segment_segment(a: LineSegment, b: LineSegment) -> bool {
    let bottom = (a.end.x - a.start.x) * (b.end.y - b.start.y) - (a.end.y - a.start.y) * (b.end.x - b.start.x);
    let top1 = (a.start.y - b.start.y) * (b.end.x - b.start.x) - (a.start.x - b.start.x) * (b.end.y - b.start.y);
    let top2 = (a.start.y - b.start.y) * (a.end.x - a.start.x) - (a.start.x - b.start.x) * (a.end.y - a.start.y);
    // Collinear
    if top1 == 0.0 && top2 == 0.0 && bottom == 0.0 {
        return true;
    }
    // Parallel and not collinear
    if bottom == 0.0 {
        return false;
    }
    let r1 = top1 / bottom;
    let r2 = top2 / bottom;
    (0.0..=1.0).contains(&r1) && (0.0..=1.0).contains(&r2)
}

pub fn rect_segment(rect: Rectangle, segment: LineSegment) -> bool {
    rect.sides().into_iter().any(|side| segment_segment(side, segment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_segment_1() {
        let s1 = LineSegment::from([[-1.0, 0.0], [1.0, 0.0]]);
        let s2 = LineSegment::from([[0.0, 1.0], [0.0, -1.0]]);
        assert!(segment_segment(s1, s2));
    }
}
