#[cfg(test)]
mod tests {
    use crate::geometry;
    use angle::Angle;

    #[test]
    fn angular_distance() {
        let max_delta = angle::Deg(0.01);
        let tests = vec![((angle::Deg(15.7), angle::Deg(96.3)), (angle::Deg(73.2), angle::Deg(93.9)), angle::Deg(57.52))];
        for ((dec_1, ra_1), (dec_2, ra_2), expected_res) in tests {
            let dec_1 = dec_1.to_rad();
            let ra_1 = ra_1.to_rad();
            let dec_2 = dec_2.to_rad();
            let ra_2 = ra_2.to_rad();
            let res = geometry::angular_distance((ra_1, dec_1), (ra_2, dec_2));
            let res = res.to_deg();
            if (res - expected_res).abs() > max_delta {
                dbg!(res, expected_res);
            }
            assert!((res - expected_res).abs() <= max_delta);
        }
    }

    #[test]
    fn vec_to_dec_ra() {
        let max_delta = angle::Deg(0.002);
        for dec in -900..=900 {
            let dec = angle::Deg((dec as f32) / 10.0);
            for ra in 0..=3600 {
                let ra = angle::Deg((ra as f32) / 10.0);
                let v = geometry::get_point_vector(ra, dec, &nalgebra::Matrix3::identity());
                let (dec_2, ra_2) = geometry::cartesian_to_spherical(v);
                let dec_2 = dec_2.to_deg();
                let ra_2 = ra_2.to_deg();
                let dec_diff = (dec - dec_2).abs();
                let ra_diff = angle::Deg(((ra.value() % 360.0) - (ra_2.value() % 360.0)).abs());
                if (ra_diff > max_delta && dec.abs() != angle::Deg(90.0)) || dec_diff > max_delta {
                    dbg!(v);
                    dbg!(ra, ra_2);
                    dbg!(dec, dec_2);
                }
                assert!(dec_diff <= max_delta);
                if dec.abs() != angle::Deg(90.0) {
                    assert!(ra_diff <= max_delta);
                }
            }
        }
    }
}
