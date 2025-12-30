pub mod cubemap;

#[derive(Default)]
pub struct Textures {
    pub clouds_texture_to_upload: Option<cubemap::Cubemap<u8>>,
}
