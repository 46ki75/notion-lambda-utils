use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Blue,
    BlueBackground,
    Brown,
    BrownBackground,
    Default,
    Gray,
    GrayBackground,
    Green,
    GreenBackground,
    Orange,
    OrangeBackground,
    Pink,
    PinkBackground,
    Purple,
    PurpleBackground,
    Red,
    RedBackground,
    Yellow,
    YellowBackground,
}
