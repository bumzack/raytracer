use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;
use crate::algebra::color::ColorOps;
use crate::utils::raytracer_error::RayTracerError;

pub struct Optics {
    matte_color: Color,
    gloss_color: Color,
    opacity: f32,
}

pub trait OpticsOps {
    fn default() -> Optics;
    fn new(matte_color: Color, glosse_color: Color, opacity: f32) -> Optics;
    fn validate_reflection_color(c: Color) -> Result<(), RayTracerError>;

    // https://stackoverflow.com/questions/35390615/writing-getter-setter-properties-in-rust
    fn set_matte_color(&mut self, m: Color);
    fn set_gloss_color(&mut self, g: Color);

    // these are immutable references - see so link for mutable?
    fn get_matte_color(&self) -> &Color;
    fn get_gloss_color(&self) -> &Color;

    fn set_matte_gloss_balance(&mut self, gloss_factor: f32, raw_matte_color: Color, raw_gloss_color: Color);
    fn set_opactiy(&mut self, o: f32);
}

impl OpticsOps for Optics {
    fn default() -> Optics {
        Optics {
            matte_color: Color::new(1.0, 1.0, 1.0),
            gloss_color: Color::new(0.0, 0.0, 0.0),
            opacity: 1.0,
        }
    }

    fn new(matte_color: Color, gloss_color: Color, opacity: f32) -> Optics {
        Optics {
            matte_color: matte_color,
            gloss_color: gloss_color,
            opacity: opacity,
        }
    }

    fn validate_reflection_color(c: Color) -> Result<(), RayTracerError>
    {
        if c.r < 0.0 || c.r > 1.0 {
            return Err::<(), RayTracerError>(RayTracerError::OpticsInvalid("red is < 0 or > 1".to_string()));
        }
        if c.g < 0.0 || c.g > 1.0 {
            return Err::<(), RayTracerError>(RayTracerError::OpticsInvalid("green is < 0 or > 1".to_string()));
        }
        if c.b < 0.0 || c.b > 1.0 {
            return Err::<(), RayTracerError>(RayTracerError::OpticsInvalid("blue is < 0 or > 1".to_string()));
        }
        Ok(())
    }

    fn set_matte_color(&mut self, m: Color) {
        self.matte_color = m;
    }

    fn set_gloss_color(&mut self, g: Color) {
        self.gloss_color = g;
    }

    fn get_matte_color(&self) -> &Color {
        &self.matte_color
    }

    fn get_gloss_color(&self) -> &Color {
        &self.gloss_color
    }

    fn set_matte_gloss_balance(&mut self, gloss_factor: f32, raw_matte_color: Color, raw_gloss_color: Color) {
        unimplemented!()
    }

    fn set_opactiy(&mut self, o: f32) {
        self.opacity = o;
    }
}

impl Display for Optics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, opacity: {}", self.matte_color, self.gloss_color, self.opacity)
    }
}

#[test]
fn test_stuff() {

// TODO  use assertions :-)
}