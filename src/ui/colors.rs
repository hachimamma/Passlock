use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Gruvbox,
    Dracula,
    Nord,
    TokyoNight,
    Catppuccin,
    Solarized,
    OneDark,
    Cyberpunk,
    HighContrastDark,
    HighContrastLight,
}

impl Theme {
    pub fn all() -> Vec<Theme> {
        vec![
            Theme::Gruvbox,
            Theme::Dracula,
            Theme::Nord,
            Theme::TokyoNight,
            Theme::Catppuccin,
            Theme::Solarized,
            Theme::OneDark,
            Theme::Cyberpunk,
            Theme::HighContrastDark,
            Theme::HighContrastLight,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Theme::Gruvbox => "Gruvbox Dark",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::TokyoNight => "Tokyo Night",
            Theme::Catppuccin => "Catppuccin Mocha",
            Theme::Solarized => "Solarized Dark",
            Theme::OneDark => "One Dark",
            Theme::Cyberpunk => "Cyberpunk",
            Theme::HighContrastDark => "High Contrast Dark",
            Theme::HighContrastLight => "High Contrast Light",
        }
    }

    pub fn from_name(name: &str) -> Theme {
        match name {
            "Dracula" => Theme::Dracula,
            "Nord" => Theme::Nord,
            "TokyoNight" | "Tokyo Night" => Theme::TokyoNight,
            "Catppuccin" | "Catppuccin Mocha" => Theme::Catppuccin,
            "SolarizedDark" | "Solarized Dark" => Theme::Solarized,
            "OneDark" | "One Dark" => Theme::OneDark,
            "Cyberpunk" => Theme::Cyberpunk,
            "HighContrastDark" | "High Contrast Dark" => Theme::HighContrastDark,
            "HighContrastLight" | "High Contrast Light" => Theme::HighContrastLight,
            _ => Theme::Gruvbox,
        }
    }

    pub fn to_config_name(self) -> String {
        match self {
            Theme::Gruvbox => "GruvboxDark",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::TokyoNight => "TokyoNight",
            Theme::Catppuccin => "Catppuccin",
            Theme::Solarized => "SolarizedDark",
            Theme::OneDark => "OneDark",
            Theme::Cyberpunk => "Cyberpunk",
            Theme::HighContrastDark => "HighContrastDark",
            Theme::HighContrastLight => "HighContrastLight",
        }
        .to_string()
    }

    pub fn next(self) -> Theme {
        match self {
            Theme::Gruvbox => Theme::Dracula,
            Theme::Dracula => Theme::Nord,
            Theme::Nord => Theme::TokyoNight,
            Theme::TokyoNight => Theme::Catppuccin,
            Theme::Catppuccin => Theme::Solarized,
            Theme::Solarized => Theme::OneDark,
            Theme::OneDark => Theme::Cyberpunk,
            Theme::Cyberpunk => Theme::HighContrastDark,
            Theme::HighContrastDark => Theme::HighContrastLight,
            Theme::HighContrastLight => Theme::Gruvbox,
        }
    }

    pub fn previous(self) -> Theme {
        match self {
            Theme::Gruvbox => Theme::HighContrastLight,
            Theme::Dracula => Theme::Gruvbox,
            Theme::Nord => Theme::Dracula,
            Theme::TokyoNight => Theme::Nord,
            Theme::Catppuccin => Theme::TokyoNight,
            Theme::Solarized => Theme::Catppuccin,
            Theme::OneDark => Theme::Solarized,
            Theme::Cyberpunk => Theme::OneDark,
            Theme::HighContrastDark => Theme::Cyberpunk,
            Theme::HighContrastLight => Theme::HighContrastDark,
        }
    }
}

#[allow(dead_code)]
pub trait ThemeColors {
    fn bg0(&self) -> Color;
    fn bg1(&self) -> Color;
    fn fg(&self) -> Color;
    fn red(&self) -> Color;
    fn green(&self) -> Color;
    fn yellow(&self) -> Color;
    fn blue(&self) -> Color;
    fn purple(&self) -> Color;
    fn aqua(&self) -> Color;
    fn orange(&self) -> Color;
    fn gray(&self) -> Color;
}

impl ThemeColors for Theme {
    fn bg0(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(40, 40, 40),
            Theme::Dracula => Color::Rgb(40, 42, 54),
            Theme::Nord => Color::Rgb(46, 52, 64),
            Theme::TokyoNight => Color::Rgb(26, 27, 38),
            Theme::Catppuccin => Color::Rgb(30, 30, 46),
            Theme::Solarized => Color::Rgb(0, 43, 54),
            Theme::OneDark => Color::Rgb(40, 44, 52),
            Theme::Cyberpunk => Color::Rgb(16, 16, 24),
            Theme::HighContrastDark => Color::Rgb(0, 0, 0),
            Theme::HighContrastLight => Color::Rgb(255, 255, 255),
        }
    }

    fn bg1(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(60, 56, 54),
            Theme::Dracula => Color::Rgb(68, 71, 90),
            Theme::Nord => Color::Rgb(59, 66, 82),
            Theme::TokyoNight => Color::Rgb(36, 40, 59),
            Theme::Catppuccin => Color::Rgb(49, 50, 68),
            Theme::Solarized => Color::Rgb(7, 54, 66),
            Theme::OneDark => Color::Rgb(53, 59, 69),
            Theme::Cyberpunk => Color::Rgb(24, 24, 37),
            Theme::HighContrastDark => Color::Rgb(20, 20, 20),
            Theme::HighContrastLight => Color::Rgb(240, 240, 240),
        }
    }

    fn fg(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(235, 219, 178),
            Theme::Dracula => Color::Rgb(248, 248, 242),
            Theme::Nord => Color::Rgb(216, 222, 233),
            Theme::TokyoNight => Color::Rgb(192, 202, 245),
            Theme::Catppuccin => Color::Rgb(205, 214, 244),
            Theme::Solarized => Color::Rgb(131, 148, 150),
            Theme::OneDark => Color::Rgb(171, 178, 191),
            Theme::Cyberpunk => Color::Rgb(0, 255, 255),
            Theme::HighContrastDark => Color::Rgb(255, 255, 255),
            Theme::HighContrastLight => Color::Rgb(0, 0, 0),
        }
    }

    fn red(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(251, 73, 52),
            Theme::Dracula => Color::Rgb(255, 85, 85),
            Theme::Nord => Color::Rgb(191, 97, 106),
            Theme::TokyoNight => Color::Rgb(247, 118, 142),
            Theme::Catppuccin => Color::Rgb(243, 139, 168),
            Theme::Solarized => Color::Rgb(220, 50, 47),
            Theme::OneDark => Color::Rgb(224, 108, 117),
            Theme::Cyberpunk => Color::Rgb(255, 0, 102),
            Theme::HighContrastDark => Color::Rgb(255, 100, 100),
            Theme::HighContrastLight => Color::Rgb(200, 0, 0),
        }
    }

    fn green(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(184, 187, 38),
            Theme::Dracula => Color::Rgb(80, 250, 123),
            Theme::Nord => Color::Rgb(163, 190, 140),
            Theme::TokyoNight => Color::Rgb(158, 206, 106),
            Theme::Catppuccin => Color::Rgb(166, 227, 161),
            Theme::Solarized => Color::Rgb(133, 153, 0),
            Theme::OneDark => Color::Rgb(152, 195, 121),
            Theme::Cyberpunk => Color::Rgb(0, 255, 157),
            Theme::HighContrastDark => Color::Rgb(100, 255, 100),
            Theme::HighContrastLight => Color::Rgb(0, 150, 0),
        }
    }

    fn yellow(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(250, 189, 47),
            Theme::Dracula => Color::Rgb(241, 250, 140),
            Theme::Nord => Color::Rgb(235, 203, 139),
            Theme::TokyoNight => Color::Rgb(224, 175, 104),
            Theme::Catppuccin => Color::Rgb(249, 226, 175),
            Theme::Solarized => Color::Rgb(181, 137, 0),
            Theme::OneDark => Color::Rgb(229, 192, 123),
            Theme::Cyberpunk => Color::Rgb(255, 255, 0),
            Theme::HighContrastDark => Color::Rgb(255, 255, 100),
            Theme::HighContrastLight => Color::Rgb(180, 140, 0),
        }
    }

    fn blue(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(131, 165, 152),
            Theme::Dracula => Color::Rgb(189, 147, 249),
            Theme::Nord => Color::Rgb(136, 192, 208),
            Theme::TokyoNight => Color::Rgb(122, 162, 247),
            Theme::Catppuccin => Color::Rgb(137, 180, 250),
            Theme::Solarized => Color::Rgb(38, 139, 210),
            Theme::OneDark => Color::Rgb(97, 175, 239),
            Theme::Cyberpunk => Color::Rgb(0, 170, 255),
            Theme::HighContrastDark => Color::Rgb(100, 150, 255),
            Theme::HighContrastLight => Color::Rgb(0, 0, 200),
        }
    }

    fn purple(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(211, 134, 155),
            Theme::Dracula => Color::Rgb(189, 147, 249),
            Theme::Nord => Color::Rgb(180, 142, 173),
            Theme::TokyoNight => Color::Rgb(187, 154, 247),
            Theme::Catppuccin => Color::Rgb(203, 166, 247),
            Theme::Solarized => Color::Rgb(108, 113, 196),
            Theme::OneDark => Color::Rgb(198, 120, 221),
            Theme::Cyberpunk => Color::Rgb(191, 0, 255),
            Theme::HighContrastDark => Color::Rgb(255, 100, 255),
            Theme::HighContrastLight => Color::Rgb(150, 0, 150),
        }
    }

    fn aqua(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(142, 192, 124),
            Theme::Dracula => Color::Rgb(139, 233, 253),
            Theme::Nord => Color::Rgb(143, 188, 187),
            Theme::TokyoNight => Color::Rgb(125, 207, 255),
            Theme::Catppuccin => Color::Rgb(148, 226, 213),
            Theme::Solarized => Color::Rgb(42, 161, 152),
            Theme::OneDark => Color::Rgb(86, 182, 194),
            Theme::Cyberpunk => Color::Rgb(0, 255, 255),
            Theme::HighContrastDark => Color::Rgb(100, 255, 255),
            Theme::HighContrastLight => Color::Rgb(0, 150, 150),
        }
    }

    fn orange(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(254, 128, 25),
            Theme::Dracula => Color::Rgb(255, 184, 108),
            Theme::Nord => Color::Rgb(208, 135, 112),
            Theme::TokyoNight => Color::Rgb(255, 158, 100),
            Theme::Catppuccin => Color::Rgb(250, 179, 135),
            Theme::Solarized => Color::Rgb(203, 75, 22),
            Theme::OneDark => Color::Rgb(209, 154, 102),
            Theme::Cyberpunk => Color::Rgb(255, 102, 0),
            Theme::HighContrastDark => Color::Rgb(255, 180, 100),
            Theme::HighContrastLight => Color::Rgb(200, 100, 0),
        }
    }

    fn gray(&self) -> Color {
        match self {
            Theme::Gruvbox => Color::Rgb(146, 131, 116),
            Theme::Dracula => Color::Rgb(98, 114, 164),
            Theme::Nord => Color::Rgb(76, 86, 106),
            Theme::TokyoNight => Color::Rgb(86, 95, 137),
            Theme::Catppuccin => Color::Rgb(108, 112, 134),
            Theme::Solarized => Color::Rgb(88, 110, 117),
            Theme::OneDark => Color::Rgb(92, 99, 112),
            Theme::Cyberpunk => Color::Rgb(128, 128, 160),
            Theme::HighContrastDark => Color::Rgb(180, 180, 180),
            Theme::HighContrastLight => Color::Rgb(100, 100, 100),
        }
    }
}

// Legacy support - keep GruvboxColors for backward compatibility
#[allow(dead_code)]
pub struct GruvboxColors;

#[allow(dead_code)]
impl GruvboxColors {
    pub fn bg0() -> Color {
        Theme::Gruvbox.bg0()
    }

    pub fn fg() -> Color {
        Theme::Gruvbox.fg()
    }

    pub fn red() -> Color {
        Theme::Gruvbox.red()
    }

    pub fn green() -> Color {
        Theme::Gruvbox.green()
    }

    pub fn yellow() -> Color {
        Theme::Gruvbox.yellow()
    }

    pub fn blue() -> Color {
        Theme::Gruvbox.blue()
    }

    pub fn purple() -> Color {
        Theme::Gruvbox.purple()
    }

    pub fn aqua() -> Color {
        Theme::Gruvbox.aqua()
    }

    pub fn orange() -> Color {
        Theme::Gruvbox.orange()
    }

    pub fn gray() -> Color {
        Theme::Gruvbox.gray()
    }
}
