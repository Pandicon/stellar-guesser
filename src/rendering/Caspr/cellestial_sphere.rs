use nalgebra::{Vector3, Vector2};
use eframe::egui;

#[derive(Deserialize)]
pub struct Star {
    pub ra:f64,
    pub dec:f64,
    pub vmag:f64
}

impl Star{
    pub fn get_unit_vector(&self)-> Vector3<f64>{
        let (ra_s,ra_c) = (-self.ra * 3.14159 / 180.0).sin_cos();
        let (de_s,de_c) = ((90.0-self.dec) * 3.14159 / 180.0).sin_cos();
        Vector3::new(de_s*ra_c, de_s*ra_s, de_c)
    }
}

pub struct Marker{
    pub normal:Vector3<f32>
}

pub struct StarRenderer{

}

pub struct CellestialSphere{
    pub stars:Vec<Star>,
    pub markers:Vec<Marker>,
    zoom:f32,
    star_renderers:Vec<StarRenderer>,
}
impl CellestialSphere{
    //Renders a circle based on its current normal(does NOT account for the rotation of the sphere)
    pub fn render_circle(&self, normal: &Vector3<f32>,radius: &f32, color:&Color32, &mut painter:Painter, ctx:&egui::Context){

        let scale_factor = 1-normal[2]/self.zoom;	

        // painter.circle_filled(egui::Pos2::new((normal[0]/(scale_factor)*400.0+420.0) as f32,(normal[1]/(scale_factor)*400.0+420.0) as f32),size_factor, color);
        
        let viewport_rect = ctx.input(|i| i.screen_rect());

        let rect_size = Vector2::new(viewport_rect.max[0]-viewport_rect.min[0],viewport_rect.max[1]-viewport_rect.min[1]);

        let screen_ratio = 2.0/(rect_size[0]*rect_size[0]+rect_size[1]*rect_size[1]).sqrt();

        let point_coordinates = Vectro2::new(normal[0]/scale_factor,normal[1]/scale_factor);

        // is it within the bounds that we want to render in? 
        if (rect_size[0]*screen_ratio/2.0>point_coordinates[0]>-rect_size[0]*screen_ratio/2.0) || (rect_size[1]*screen_ratio/2.0>point_coordinates[1]>-rect_size[1]*screen_ratio/2.0) {

            painter.circle_filled(egui::Pos2::new(point_coordinates[0]/screen_ratio+rect_size[0]/2,point_coordinates[1]/screen_ratio+rect_size[1]/2),size_factor, color);
        }
 
    }

    //Renders the entire sphere view
    pub fn render_sky(&self, &mut painter:Painter, ctx:&egui::Context){
        
    }

}