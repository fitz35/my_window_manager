use penrose::x::XConn;
use penrose::Color;
use penrose_ui::bar::widgets::{amixer_volume, battery_summary, current_date_and_time, wifi_network, ActiveWindowName, CurrentLayout, Workspaces};
use penrose_ui::{Position, StatusBar, TextStyle};

use crate::styles::bar_styles::{BAR_FONT_SIZE, BAR_HEIGHT_PX, MAX_ACTIVE_WINDOW_CHARS};
use crate::styles::colors::CustomColor;
use crate::styles::PROFONT;





pub fn status_bar<X: XConn>() -> StatusBar<X> {
    let highlight: Color = CustomColor::dark_blue().into();
    let empty_ws: Color = CustomColor::grey().into();

    let style = TextStyle {
        fg: CustomColor::white().into(),
        bg: Some(CustomColor::black().into()),
        padding: (2, 2),
    };

    let padded_style = TextStyle {
        padding: (4, 2),
        ..style
    };

    StatusBar::try_new(
        Position::Bottom,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        PROFONT,
        BAR_FONT_SIZE,
        vec![
            Box::new(Workspaces::new(style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(style)),
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                TextStyle {
                    bg: Some(highlight),
                    padding: (6, 4),
                    ..style
                },
                true,
                false,
            )),
            Box::new(wifi_network(padded_style)),
            Box::new(battery_summary("BAT0", padded_style)),
            Box::new(amixer_volume("Master", padded_style)),
            Box::new(current_date_and_time(padded_style)),
        ],
    ).unwrap()
}