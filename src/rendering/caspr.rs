use nalgebra::{Vector3, Vector2};
use eframe::egui;
use serde::Deserialize;
use std::{error::Error, f32::consts::PI, fs};


#[derive(Clone, Copy,Deserialize)]
pub struct Star {
    pub ra: f32,
    pub dec: f32,
    pub vmag: f32
}

impl Star{
    pub fn get_renderer(&self)-> StarRenderer{
        let (ra_s, ra_c) = (-self.ra * PI / 180.0).sin_cos();
        let (de_s, de_c) = ((90.0-self.dec) * PI / 180.0).sin_cos();
        StarRenderer::new(Vector3::new(de_s*ra_c, de_s*ra_s, de_c),self.vmag)
    }
}

pub struct Marker{
    pub normal: Vector3<f32>
}

pub struct StarRenderer{
    pub unit_vector: Vector3<f32>,
    pub vmag: f32

}
impl StarRenderer {
    pub fn new(vector: Vector3<f32>, magnitude: f32) -> Self{
        Self { unit_vector: vector, vmag: magnitude }
    }
    pub fn render(&self,cellestial_sphere:&CellestialSphere , painter:&mut egui::Painter, ctx:&egui::Context){
        cellestial_sphere.render_circle(&self.unit_vector, cellestial_sphere.mag_to_radius(self.vmag), cellestial_sphere.star_color, painter, ctx);
    }
    
}

pub struct CellestialSphere{
    pub stars: Vec<Star>,
    pub markers: Vec<Marker>,
    zoom: f32,
    star_renderers: Vec<StarRenderer>,
    mag_scale: f32,
    mag_offset: f32,
    star_color: eframe::epaint::Color32
}
impl CellestialSphere{
    //Renders a circle based on its current normal(does NOT account for the rotation of the sphere)
    pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: eframe::epaint::Color32, painter: &mut egui::Painter, ctx: &egui::Context){

        let scale_factor = 1.0-normal[2]/self.zoom;	
        
        let viewport_rect = ctx.input(|i| i.screen_rect());

        let rect_size = Vector2::new(viewport_rect.max[0]-viewport_rect.min[0],viewport_rect.max[1]-viewport_rect.min[1]);

        let screen_ratio = 2.0/(rect_size[0]*rect_size[0]+rect_size[1]*rect_size[1]).sqrt();

        let point_coordinates = Vector2::new(normal[0]/scale_factor,normal[1]/scale_factor);

        // is it within the bounds that we want to render in? 
        if ((rect_size[0]*screen_ratio/2.0 > point_coordinates[0]) && (point_coordinates[0] > -rect_size[0]*screen_ratio/2.0)) || ((rect_size[1]*screen_ratio/2.0 > point_coordinates[1]) &&(point_coordinates[1] > -rect_size[1]*screen_ratio/2.0)) {

            painter.circle_filled(egui::Pos2::new(point_coordinates[0]/screen_ratio+rect_size[0]/2.0,point_coordinates[1]/screen_ratio+rect_size[1]/2.0), radius, color);
        }
 
    }

    //Renders the entire sphere view
    pub fn render_sky(&self, painter: &mut egui::Painter, ctx: &egui::Context){
        //some stuff lol
        for star_renderer in &self.star_renderers{
            star_renderer.render(&self,painter, ctx)
        }
    }

    pub fn load() -> Result<Self, Box<dyn Error>>{
        let mut catalog:Vec<Star> = Vec::new();

        for data in fs::read_dir("./sphere/stars") {
            let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path("data.csv");
    
            for star in reader?.deserialize() {
                let star: Star = star?;
                catalog.push(star);
            }
        }

        Ok(Self { stars: catalog, markers: Vec::new(), zoom: 1.0,  star_renderers: Vec::new(),mag_scale:0.3,mag_offset:6.0,star_color:eframe::epaint::Color32::WHITE})

    }
    pub fn init(&mut self){
        self.star_renderers = self.stars.iter().map(|i| i.get_renderer()).collect()
    }

    pub fn mag_to_radius(&self,vmag:f32)-> f32{
        self.mag_scale*(self.mag_offset-vmag)
    }

}