use super::{LineSegment, Rectangle};

pub fn bounding_boxes_segment_segment(a: LineSegment, b: LineSegment) -> bool {
    // This check requires the start points to be the bottom left corners of the bounding boxes and the end points to be the top right corners
    let a = LineSegment::from([[a.start.x.min(a.end.x), a.start.y.min(a.end.y)], [a.start.x.max(a.end.x), a.start.y.max(a.end.y)]]);
    let b = LineSegment::from([[b.start.x.min(b.end.x), b.start.y.min(b.end.y)], [b.start.x.max(b.end.x), b.start.y.max(b.end.y)]]);
    a.start.x <= b.end.x && a.end.x >= b.start.x && a.start.y <= b.end.y && a.end.y >= b.start.y
}

pub fn segment_segment(a: LineSegment, b: LineSegment) -> bool {
    // Bounding boxes do not intersect -> the lines also can not intersect
    if !bounding_boxes_segment_segment(a, b) {
        return false;
    }
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

    // The segment-segment intersection tests were taken from https://martin-thoma.com/how-to-check-if-two-line-segments-intersect/#test-cases (http://web.archive.org/web/20240315225152/https://martin-thoma.com/how-to-check-if-two-line-segments-intersect/#test-cases)
    // The numbers correspond to those on that website, with some tests being added as modifications (see for example the t_2_2 test using coordinates that are not as nicely divisible)
    #[test]
    fn segment_segment_t_1() {
        let s1 = LineSegment::from([[-4.0, 0.0], [4.0, 0.0]]);
        let s2 = LineSegment::from([[0.0, -4.0], [0.0, 4.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_t_2_1() {
        let s1 = LineSegment::from([[0.0, 0.0], [10.0, 10.0]]);
        let s2 = LineSegment::from([[2.0, 2.0], [16.0, 4.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_t_2_2() {
        let s1 = LineSegment::from([[0.0, 0.0], [10.0, 10.0]]);
        let s2 = LineSegment::from([[2.0, 2.0], [17.0, 4.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_t_3() {
        let s1 = LineSegment::from([[-2.0, 0.0], [0.0, 0.0]]);
        let s2 = LineSegment::from([[0.0, -2.0], [0.0, 2.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_t_4() {
        let s1 = LineSegment::from([[0.0, 4.0], [4.0, 4.0]]);
        let s2 = LineSegment::from([[4.0, 0.0], [4.0, 8.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_t_5() {
        let s1 = LineSegment::from([[0.0, 0.0], [10.0, 10.0]]);
        let s2 = LineSegment::from([[2.0, 2.0], [6.0, 6.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_t_6() {
        let s1 = LineSegment::from([[6.0, 8.0], [14.0, -2.0]]);
        let s2 = LineSegment::from([[6.0, 8.0], [14.0, -2.0]]);
        assert!(segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_1() {
        let s1 = LineSegment::from([[4.0, 4.0], [12.0, 12.0]]);
        let s2 = LineSegment::from([[6.0, 8.0], [8.0, 10.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_2() {
        let s1 = LineSegment::from([[-8.0, 8.0], [-4.0, 2.0]]);
        let s2 = LineSegment::from([[-4.0, 6.0], [0.0, 0.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_3() {
        let s1 = LineSegment::from([[0.0, 0.0], [0.0, 2.0]]);
        let s2 = LineSegment::from([[4.0, 4.0], [4.0, 6.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_4() {
        let s1 = LineSegment::from([[0.0, 0.0], [0.0, 2.0]]);
        let s2 = LineSegment::from([[4.0, 4.0], [6.0, 4.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_5() {
        let s1 = LineSegment::from([[-2.0, -2.0], [4.0, 4.0]]);
        let s2 = LineSegment::from([[6.0, 6.0], [10.0, 10.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_6() {
        let s1 = LineSegment::from([[0.0, 0.0], [2.0, 2.0]]);
        let s2 = LineSegment::from([[1.0, 4.0], [4.0, 0.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_7() {
        let s1 = LineSegment::from([[2.0, 2.0], [8.0, 2.0]]);
        let s2 = LineSegment::from([[4.0, 4.0], [6.0, 4.0]]);
        assert!(!segment_segment(s1, s2));
    }

    #[test]
    fn segment_segment_f_8() {
        let s1 = LineSegment::from([[0.0, 8.0], [10.0, 0.0]]);
        let s2 = LineSegment::from([[4.0, 2.0], [4.0, 4.0]]);
        assert!(!segment_segment(s1, s2));
    }
}
