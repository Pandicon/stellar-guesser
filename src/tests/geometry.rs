#[path = "../geometry.rs"]
mod geometry;

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;

	use super::geometry::*;

	#[test]
	fn vec_to_dec_ra() {
		let max_delta = 0.002;
		for dec in -900..=900 {
			let dec = (dec as f32) / 10.0;
			for ra in 0..=3600 {
				let ra = (ra as f32) / 10.0;
				let v = get_point_vector(ra, dec, &nalgebra::Matrix3::identity());
				let (dec_2, ra_2) = cartesian_to_spherical(v);
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
