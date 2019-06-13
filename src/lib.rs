mod fov_dummy;
mod fov_recursive_shadowcasting;
mod fov_restrictive;

pub use fov_dummy::*;
pub use fov_recursive_shadowcasting::*;
pub use fov_restrictive::*;

/// Some basic structure to store map cells' transparency and fov computation result
pub struct MapData {
    /// width of the map in cells
    pub width: usize,
    /// height of the map in cells
    pub height: usize,
    /// width x height vector of transparency information
    pub transparent: Vec<bool>,
    /// width x height vector of field of view information
    pub fov: Vec<bool>,
}

impl MapData {
    /// create a new empty map : no walls and empty field of view
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            transparent: vec![true;width*height],
            fov:vec![false;width*height],
        }
    }
    /// reset the fov information to false
    pub fn clear_fov(&mut self) {
        for off in 0..self.width*self.height {
            self.fov[off] = false;
        }
    }
    pub fn is_in_fov(&self, x: usize, y: usize) -> bool {
        self.fov[x + y*self.width]
    }
    pub fn is_transparent(&self, x: usize, y: usize) -> bool {
        self.transparent[x + y*self.width]
    }
    pub fn set_fov(&mut self, x: usize, y: usize, in_fov: bool) {
        self.fov[x + y*self.width] = in_fov;
    }
    pub fn set_transparent(&mut self, x: usize, y: usize, is_transparent: bool) {
        self.transparent[x + y*self.width] = is_transparent;
    }
}

/// Some algorithm to compute a field of view
/// x,y : observer position on the map
/// max_radius : max distance in cells where the observer can see. 0 = infinite
/// light_walls : are walls limiting the field of view inside the field of view ?
pub trait FovAlgorithm {
    fn compute_fov(&mut self, map: &mut MapData, x: usize, y: usize,
        max_radius: usize, light_walls: bool);
}

#[cfg(test)]
mod tests {
    use crate::{FovAlgorithm, FovDummy, FovRecursiveShadowCasting, FovRestrictive, MapData};

    #[test]
    fn fov_dummy() {
        let mut fov = FovDummy::new();
        let mut map = MapData::new(10, 10);
        map.set_transparent(5,5,false);
        fov.compute_fov(&mut map, 0, 0, 0, false);
        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(map.transparent[x + y * 10], map.fov[x + y*10]);
            }
        }
    }

    #[test]
    fn fov_shadowcasting() {
        let mut fov = FovRecursiveShadowCasting::new();
        let mut map = MapData::new(10, 10);
        map.set_transparent(5,5,false);
        fov.compute_fov(&mut map, 5, 6, 0, false);
        assert_eq!(map.is_in_fov(5,6), true);
        assert_eq!(map.is_in_fov(5,7), true);
        assert_eq!(map.is_in_fov(5,5), false);
        assert_eq!(map.is_in_fov(5,4), false);
    }

    #[test]
    fn fov_mrpas() {
        let mut fov = FovRestrictive::new();
        let mut map = MapData::new(10, 10);
        map.set_transparent(5,5,false);
        fov.compute_fov(&mut map, 5, 6, 0, false);
        assert_eq!(map.is_in_fov(5,6), true);
        assert_eq!(map.is_in_fov(5,7), true);
        assert_eq!(map.is_in_fov(5,5), false);
        assert_eq!(map.is_in_fov(5,4), false);
    }
}
