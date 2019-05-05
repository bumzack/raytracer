use crate::utils::raytracer_error::RayTracerError;

const REFRACTION_VACUUM: f32 = 1.0000;
const REFRACTION_AIR: f32 = 1.0003;
const REFRACTION_ICE: f32 = 1.3100;
const REFRACTION_WATER: f32 = 1.3330;
const REFRACTION_GASOLINE: f32 = 1.3980;
const REFRACTION_GLASS: f32 = 1.5500;
const REFRACTION_SAPPHIRE: f32 = 1.7700;
const REFRACTION_DIAMOND: f32 = 2.4190;

// Range of allowed refraction values...
const REFRACTION_MINIMUM: f32 = 1.0000;
const REFRACTION_MAXIMUM: f32 = 9.0000;


pub fn validate_refraction(refraction: f32) -> Result<(), RayTracerError>
{
    if refraction < REFRACTION_MINIMUM ||
        refraction > REFRACTION_MAXIMUM {
        return Err::<(), RayTracerError>(RayTracerError::RefractionInvalid("refraction is not within MIN/MAX bounds".to_string()));
    }
    Ok(())
}

#[test]
fn test_stuff() {

// TODO  use assertions :-)
}