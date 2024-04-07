use std::fs;
use std::iter::Iterator;

mod star;
mod planet;

mod constants {
    use std::fs;
    use once_cell::sync::Lazy;

    pub static STAR_NAMES: Lazy<Vec<String>> = Lazy::new(|| {
        fs::read_to_string("./assets/namelists/star_namelist.txt").unwrap
        ().split('\n').map(|s| s.to_string()).collect()
    });
}

pub enum CelestialBodyType {
    Star,
    Planet,
    Moon,
    Asteroid,
    Comet,
    GasGiant,
    Nebula,
}

/// `CelestialBody` is a trait that represents a celestial body in the game
/// and provides methods for getting information about the body and performing
/// actions on it.
pub trait CelestialBody {
    
    /// Get the name of the celestial body that will be displayed in the UI
    /// 
    /// # Arguments
    /// * `self` - A reference to the celestial body
    /// 
    /// # Returns
    /// * String - The name of the celestial body
    fn get_name(&self) -> String;
    
    /// Get the type of the celestial body
    /// 
    /// # Arguments
    /// * `self` - A reference to the celestial body
    /// 
    /// # Returns
    /// * CelestialBodyType - The type of the celestial body
    fn get_type(&self) -> CelestialBodyType;
    
    /// Get the menu color of the celestial body
    /// 
    /// # Arguments
    /// * `self` - A reference to the celestial body
    /// 
    /// # Returns
    /// * ratatui::style::Color - The menu color of the celestial body
    fn get_menu_color(&self) -> ratatui::style::Color { ratatui::style::Color::White }
    
    /// Generate a new instance of the celestial body based on a host body if nescessary
    /// 
    /// # Arguments
    /// * `host` - An optional reference to the host body
    /// 
    /// # Returns
    /// * `Self` - A generated instance of the implementer
    fn generate<T: CelestialBody>(host: Option<T>) -> Self;
}