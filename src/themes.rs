use tui::style::Color;

pub struct Theme {
    pub title_color: Color,
    pub dark_square_color: Color,
    pub light_square_color: Color,
    pub highlighted_color: Color,
    pub error_color: Color,
    pub text_color: Color,
    pub light_number_color: Color,
    pub dark_number_color: Color,
    pub victory_color: Color
}

pub const TRANQUIL: Theme = Theme {
    title_color: Color::Rgb(170, 143, 121), // Pastel Brown
    dark_square_color: Color::Gray,
    light_square_color: Color::White,
    highlighted_color: Color::Rgb(184, 255, 184), // Mint
    error_color: Color::Rgb(255, 165, 161), // Pastel Red
    text_color: Color::White,
    light_number_color: Color::Black,
    dark_number_color: Color::Black,
    victory_color: Color::Rgb(184, 255, 184), // Mint
};