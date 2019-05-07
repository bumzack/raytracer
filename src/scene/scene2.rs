use image::{ImageBuffer, RgbImage};
use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;
use crate::algebra::color::ColorOps;
use crate::algebra::intersection::IntersectionList;
use crate::algebra::refraction::validate_refraction;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;
use crate::geometry::pixelcoordinates::PixelCoordinates;
use crate::geometry::pixeldata::PixelData;
use crate::geometry::sphere::Sphere;
use crate::scene::lightsource::LightSource;

const MAX_OPTICAL_RECURSION_DEPTH: i32 = 20;
const MIN_OPTICAL_INTENSITY: f32 = 0.001;


pub struct Scene {
    background_color: Color,
    solid_object_list: Vec<Sphere>,
    light_source_list: Vec<LightSource>,
    ambient_refraction: f32,
    cached_intersection_list: IntersectionList,
    width: usize,
    height: usize,
}

impl Scene {
    pub fn new(width: usize, height: usize, background_color: Color) -> Scene {
        Scene {
            width: width,
            height: height,
            background_color: background_color,
            solid_object_list: Vec::new(),
            light_source_list: Vec::new(),
            ambient_refraction: 1.0,
            cached_intersection_list: Vec::new(),
        }
    }


    pub fn set_ambient_refraction(&mut self, r: f32) {
        validate_refraction(r).expect("crap - refraction invalid");
        self.ambient_refraction = r;
    }

    pub fn save_image(self, filename: String) {
        let mut image: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        // TODO: write pixels to image
        image.save("sphere.png").unwrap();
    }

    pub fn just_do_it(&mut self, width: usize, height: usize, zoom: f32, anti_alias_factor: usize) {
        let large_pixels_wide = anti_alias_factor * width;
        let large_pixels_height = anti_alias_factor * height;

        let mut smaller_dim;
        if large_pixels_wide < large_pixels_height {
            smaller_dim = large_pixels_wide
        } else {
            smaller_dim = large_pixels_height;
        }
        let large_zoom = anti_alias_factor as f32 * zoom * smaller_dim as f32;

        let camera = Vector::default();
        let direction = Vector::new(0.0, 0.0, -1.0);
        let full_intensity = Color::new(1.0, 1.0, 1.0);

        let ambigious_pixels: Vec<PixelCoordinates> = Vec::new();

        // TODO: should be generic and sphere ...
        let solid_object_list: Vec<Sphere> = Vec::new();
        let light_source_list: Vec<LightSource> = Vec::new();
        let ambient_refraction = 1.2;
        let cached_intersection_list: IntersectionList = Vec::new();
    }


    // Oversample the image using the anti-aliasing factor.

//        for (size_t i=0; i < largePixelsWide; ++i)
//        {
//            direction.x = (i - largePixelsWide/2.0) / largeZoom;
//            for (size_t j=0; j < largePixelsHigh; ++j)
//            {
//                direction.y = (largePixelsHigh/2.0 - j) / largeZoom;
//

//
//                PixelData& pixel = buffer.Pixel(i,j);
//                try
//                    {
//                        // Trace a ray from the camera toward the given direction
//                        // to figure out what color to assign to this pixel.
//                        pixel.color = TraceRay(
//                            camera,
//                            direction,
//                            ambientRefraction,
//                            fullIntensity,
//                            0);
//                    }
//                catch (AmbiguousIntersectionException)
//                {
//                    // Getting here means that somewhere in the recursive
//                    // code for tracing rays, there were multiple
//                    // intersections that had minimum distance from a
//                    // vantage point.  This can be really bad,
//                    // for example causing a ray of light to reflect
//                    // inward into a solid.
//
//                    // Mark the pixel as ambiguous, so that any other
//                    // ambiguous pixels nearby know not to use it.
//                    pixel.isAmbiguous = true;
//
//                    // Keep a list of all ambiguous pixel coordinates
//                    // so that we can rapidly enumerate through them
//                    // in the disambiguation pass.
//                    ambiguousPixelList.push_back(PixelCoordinates(i, j));
//                }
//            }
//        }

//
//        // Go back and "heal" ambiguous pixels as best we can.
//        PixelList::const_iterator iter = ambiguousPixelList.begin();
//        PixelList::const_iterator end  = ambiguousPixelList.end();
//        for (; iter != end; ++iter)
//        {
//            const PixelCoordinates& p = *iter;
//            ResolveAmbiguousPixel(buffer, p.i, p.j);
//        }
//
//        // We want to scale the arbitrary range of
//        // color component values to the range 0..255
//        // allowed by PNG format.  We therefore find
//        // the maximum red, green, or blue value anywhere
//        // in the image.
//        const double max = buffer.MaxColorValue();
//
//        // Downsample the image buffer to an integer array of RGBA
//        // values that LodePNG understands.
//        const unsigned char OPAQUE_ALPHA_VALUE = 255;
//        const unsigned BYTES_PER_PIXEL = 4;
//
//        // The number of bytes in buffer to be passed to LodePNG.
//        const unsigned RGBA_BUFFER_SIZE =
//            pixelsWide * pixelsHigh * BYTES_PER_PIXEL;
//
//        std::vector<unsigned char> rgbaBuffer(RGBA_BUFFER_SIZE);
//        unsigned rgbaIndex = 0;
//        const double patchSize = antiAliasFactor * antiAliasFactor;
//        for (size_t j=0; j < pixelsHigh; ++j)
//        {
//            for (size_t i=0; i < pixelsWide; ++i)
//            {
//                Color sum(0.0, 0.0, 0.0);
//                for (size_t di=0; di < antiAliasFactor; ++di)
//                {
//                    for (size_t dj=0; dj < antiAliasFactor; ++dj)
//                    {
//                        sum += buffer.Pixel(
//                            antiAliasFactor*i + di,
//                            antiAliasFactor*j + dj).color;
//                    }
//                }
//                sum /= patchSize;
//
//                // Convert to integer red, green, blue, alpha values,
//                // all of which must be in the range 0..255.
//                rgbaBuffer[rgbaIndex++] = ConvertPixelValue(sum.red,   max);
//                rgbaBuffer[rgbaIndex++] = ConvertPixelValue(sum.green, max);
//                rgbaBuffer[rgbaIndex++] = ConvertPixelValue(sum.blue,  max);
//                rgbaBuffer[rgbaIndex++] = OPAQUE_ALPHA_VALUE;
//            }
//        }
//
//        // Write the PNG file
//        const unsigned error = lodepng::encode(
//            outPngFileName,
//            rgbaBuffer,
//            pixelsWide,
//            pixelsHigh);
//
//        // If there was an encoding error, throw an exception.
//        if (error != 0)
//        {
//            std::string message = "PNG encoder error: ";
//            message += lodepng_error_text(error);
//            throw ImagerException(message.c_str());
//        }    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add all fields
        write!(f, "width = {}, height = {}", self.width, self.height)
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}
