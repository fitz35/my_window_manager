pub mod colors {
    use penrose::Color;

    pub struct CustomColor {
        pub hex: String,
    }

    impl CustomColor {
        pub fn black() -> Self {
            "#000000".into()
        }

        pub fn grey() -> Self {
            "#808080".into()
        }

        pub fn white() -> Self {
            "#ffffff".into()
        }

        pub fn purple() -> Self {
            "#a020f0".into()
        }

        pub fn blue() -> Self {
            "#0000ff".into()
        }

        pub fn red() -> Self {
            "#ff0000".into()
        }


        pub fn dark_blue() -> Self {
            "#341eff".into()
        }

        pub fn cyan() -> Self {
            "#0B7A75".into()
        }
    }

    impl From<&str> for CustomColor {
        fn from(hex: &str) -> Self {
            CustomColor {
                hex: hex.to_string(),
            }
        }
    }

    impl Into<Color> for CustomColor {
        fn into(self) -> Color {
            Color::try_from(self.hex).unwrap()
        }
    }
}

pub mod bar_styles {
    pub const MAX_ACTIVE_WINDOW_CHARS: usize = 40;
    pub const BAR_FONT_SIZE: u8 = 12;
    pub const BAR_HEIGHT_PX: u32 = 18;
}

pub const PROFONT: &str = "JetBrainsMono Nerd Font";