#[path = "../geometry.rs"]
mod geometry;

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;

	use super::geometry;

	#[test]
	fn angular_distance() {
		let max_delta = 0.01;
		let tests = vec![((15.7, 96.3), (73.2, 93.9), 57.52)];
		for ((dec_1, ra_1), (dec_2, ra_2), expected_res) in tests {
			let dec_1 = dec_1 * PI / 180.0;
			let ra_1 = ra_1 * PI / 180.0;
			let dec_2 = dec_2 * PI / 180.0;
			let ra_2 = ra_2 * PI / 180.0;
			let res = geometry::angular_distance((ra_1, dec_1), (ra_2, dec_2));
			let res = res * 180.0 / PI;
			if (res - expected_res).abs() > max_delta {
				dbg!(res, expected_res);
			}
			assert!((res - expected_res).abs() <= max_delta);
		}
	}

	#[test]
	fn vec_to_dec_ra() {
		let max_delta = 0.002;
		for dec in -900..=900 {
			let dec = (dec as f32) / 10.0;
			for ra in 0..=3600 {
				let ra = (ra as f32) / 10.0;
				let v = geometry::get_point_vector(ra, dec, &nalgebra::Matrix3::identity());
				let (dec_2, ra_2) = geometry::cartesian_to_spherical(v);
				let dec_2 = dec_2 * 180.0 / PI;
				let ra_2 = ra_2 * 180.0 / PI;
				if ((ra - ra_2).abs() > max_delta && dec.abs() != 90.0) || (dec - dec_2).abs() > max_delta {
					dbg!(v);
					dbg!(ra, ra_2);
					dbg!(dec, dec_2);
				}
				assert!((dec - dec_2).abs() <= max_delta);
				if dec.abs() != 90.0 {
					assert!((ra - ra_2).abs() <= max_delta);
				}
			}
		}
	}
}
