use crate::data::Radial;

/// A single radar sweep composed of a series of radials. This represents a full rotation of the
/// radar at some elevation angle and contains the Level II data (reflectivity, velocity, and
/// spectrum width) for each azimuth angle in that sweep. The resolution of the sweep dictates the
/// azimuthal distance between rays and thus and number of rays in the sweep. Multiple sweeps are
/// taken at different elevation angles to create a volume scan.
pub struct Sweep {
    elevation_number: u8,
    radials: Vec<Radial>,
}

impl Sweep {
    /// The index number for this radial's elevation in the volume scan. The precise elevation angle
    /// varies and can be found in individual radials.
    pub fn elevation_number(&self) -> u8 {
        self.elevation_number
    }

    /// The radials comprising this sweep.
    pub fn radials(&self) -> &Vec<Radial> {
        self.radials.as_ref()
    }
}