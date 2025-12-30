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
