use crate::rendering::caspr::textures;

#[derive(Clone)]
pub struct Cubemap<T> {
    pub texture_size: u32,
    pub texture_data: [textures::rectangle::Rectangle<T>; 6],

    pub changed: bool,
}

impl<T> Default for Cubemap<T> {
    fn default() -> Self {
        Self {
            texture_size: 0,
            texture_data: [
                textures::rectangle::Rectangle::<T>::empty(),
                textures::rectangle::Rectangle::<T>::empty(),
                textures::rectangle::Rectangle::<T>::empty(),
                textures::rectangle::Rectangle::<T>::empty(),
                textures::rectangle::Rectangle::<T>::empty(),
                textures::rectangle::Rectangle::<T>::empty(),
            ],

            changed: false,
        }
    }
}

impl<T> Cubemap<T> {
    pub fn pixel_to_sphere_dir(face_index: u8, x: u32, y: u32, width: u32, height: u32) -> nalgebra::Vector3<f32> {
        let u = 2.0 * (x as f32 + 0.5) / width as f32 - 1.0;
        let v = 2.0 * (y as f32 + 0.5) / height as f32 - 1.0;

        // u goes Left->Right, v goes Top->Bottom
        let cube_vec = match face_index {
            0 => nalgebra::Vector3::new(1.0, -v, -u),   // +X (Right)
            1 => nalgebra::Vector3::new(-1.0, -v, u),   // -X (Left)
            2 => nalgebra::Vector3::new(u, 1.0, v),     // +Y (Top)
            3 => nalgebra::Vector3::new(u, -1.0, -v),   // -Y (Bottom)
            4 => nalgebra::Vector3::new(u, -v, 1.0),    // +Z (Front)
            5 => nalgebra::Vector3::new(-u, -v, -1.0),  // -Z (Back)
            _ => nalgebra::Vector3::new(1.0, 1.0, 1.0), // Should not happen
        };

        cube_vec.normalize()
    }
}
