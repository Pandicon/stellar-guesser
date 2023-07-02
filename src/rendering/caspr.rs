use nalgebra::{Vector3, Vector2};
use eframe::egui;
use serde::Deserialize;
use std::{error::Error, f32::consts::PI, fs};

const STARS_FOLDER: &str = "./sphere/stars";

#[path = "../geometry.rs"]
mod geometry;

#[derive(Clone, Copy,Deserialize)]
pub struct Star {
    pub ra: f32,
    pub dec: f32,
    pub vmag: f32
}

impl Star{
    pub fn get_renderer(&self, rotation_de: f32, rotation_ra: f32) -> StarRenderer {
        let (ra_s, ra_c) = ((-self.ra + rotation_ra) * PI / 180.0).sin_cos();
        let (de_s, de_c) = ((90.0 - self.dec + rotation_de) * PI / 180.0).sin_cos();
        StarRenderer::new(Vector3::new(de_s*ra_c, de_s*ra_s, de_c), self.vmag)
    }
}

pub struct Marker{
    pub normal: Vector3<f32>
}

pub struct StarRenderer {
    pub unit_vector: Vector3<f32>,
    pub vmag: f32

}
impl StarRenderer {
    pub fn new(vector: Vector3<f32>, magnitude: f32) -> Self{
        Self { unit_vector: vector, vmag: magnitude }
    }

    pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
        cellestial_sphere.render_circle(&self.unit_vector, cellestial_sphere.mag_to_radius(self.vmag), cellestial_sphere.star_color, painter);
    }
    
}

pub struct CellestialSphere {
    pub stars: Vec<Star>,
    pub markers: Vec<Marker>,
    zoom: f32,
    star_renderers: Vec<StarRenderer>,
    mag_scale: f32,
    mag_offset: f32,
    star_color: eframe::epaint::Color32,
    pub viewport_rect: egui::Rect,

    pub rotation_dec: f32,
    pub rotation_ra: f32
}

impl CellestialSphere {
    //Renders a circle based on its current normal (does NOT account for the rotation of the sphere)
    pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: eframe::epaint::Color32, painter: &egui::Painter) {
        let scale_factor = 1.0-normal[2]/self.zoom;
        
        let viewport_rect = self.viewport_rect;

        let rect_size = Vector2::new(viewport_rect.max[0]-viewport_rect.min[0],viewport_rect.max[1]-viewport_rect.min[1]);

        let screen_ratio = 2.0/(rect_size[0]*rect_size[0]+rect_size[1]*rect_size[1]).sqrt();

        let point_coordinates = Vector2::new(normal[0]/scale_factor,normal[1]/scale_factor);

        // Is it within the bounds that we want to render in? //TODO: Use the geometry::is_in_rect function
        // TODO: Probably fix this - see how it is rendering into the top panel
        if ((rect_size[0]*screen_ratio/2.0 > point_coordinates[0]) && (point_coordinates[0] > -rect_size[0]*screen_ratio/2.0)) || ((rect_size[1]*screen_ratio/2.0 > point_coordinates[1]) && (point_coordinates[1] > -rect_size[1]*screen_ratio/2.0)) {
            painter.circle_filled(egui::Pos2::new(point_coordinates[0]/screen_ratio+rect_size[0]/2.0,point_coordinates[1]/screen_ratio+rect_size[1]/2.0), radius, color);
        }
 
    }

    //Renders the entire sphere view
    pub fn render_sky(&self, painter: &egui::Painter) {
        //some stuff lol
        for star_renderer in &self.star_renderers {
            star_renderer.render(&self, painter)
        }
    }

    pub fn load() -> Result<Self, Box<dyn Error>>{
        let mut catalog: Vec<Star> = Vec::new();
        let files = fs::read_dir(STARS_FOLDER);
        for file in files? {
            if let Ok(file) = file {
                let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());
    
                for star in reader?.deserialize() {
                    let star: Star = star?;
                    catalog.push(star);
                }
            }
        }

        let viewport_rect = egui::Rect::from_two_pos(egui::pos2(0.0, 0.0), egui::pos2(0.0, 0.0));
        Ok(Self { stars: catalog, markers: Vec::new(), zoom: 1.0, star_renderers: Vec::new(), mag_scale: 0.3, mag_offset: 6.0, star_color: eframe::epaint::Color32::WHITE, viewport_rect, rotation_dec: 0.0, rotation_ra: 0.0})
    }

    // TODO: Make this always for example halve the FOV
    pub fn zoom(&mut self, velocity: f32) {
        self.zoom += velocity;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn init(&mut self) {
        self.init_renderers();
    }

    pub fn init_renderers(&mut self) {
        self.star_renderers = self.stars.iter().map(|i| i.get_renderer(self.rotation_dec, self.rotation_ra)).collect()
    }

    pub fn mag_to_radius(&self,vmag:f32)-> f32{
        self.mag_scale*(self.mag_offset-vmag)
    }

}