#![allow(dead_code)]

use std::error::Error;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Orange,
    White
}

impl Color {
    const VALUES: [Self; 6] = [ Self::Red,
                                Self::Green,
                                Self::Yellow,
                                Self::Blue,
                                Self::Orange,
                                Self::White ];

    fn from_str(str : &str) -> Result<Color, Box<dyn Error>>{
        match str {
            "Red" => Ok(Self::Red) ,
            "Green" => Ok(Self::Green),
            "Yellow" => Ok(Self::Yellow),
            "Blue" => Ok(Self::Blue),
            "Orange" => Ok(Self::Orange),
            "White" => Ok(Self::White),
            _ => Err("unknown color".into()),
        }
    }
}

/**
 * Face : idx range
 * Front : [0, 8]
 * Back : [9, 17]
 * Left : [18, 26]
 * Right : [27, 35]
 * Top : [36, 44]
 * Bottom : [45, 53]
 */


#[derive(Eq, PartialEq, Copy, Clone)]
enum Face {
    Front,
    Back,
    Top,
    Bottom,
    Left,
    Right
}

impl Face {
    const VALUES: [Self; 6] = [ Self::Front,
                                Self::Back,
                                Self::Top,
                                Self::Bottom,
                                Self::Left,
                                Self::Right ];

    fn offset(self) -> usize {
        match self {
            Self::Front  => 0,
            Self::Back   => 9,
            Self::Left   => 18,
            Self::Right  => 27,
            Self::Top    => 36,
            Self::Bottom => 45
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Front  => Color::Red,
            Self::Back   => Color::Orange,
            Self::Left   => Color::Green,
            Self::Right  => Color::Blue,
            Self::Top    => Color::White,
            Self::Bottom => Color::Yellow,
        }
    }

    fn from_str(str : &str) -> Result<Face, Box<dyn Error>>{
        match str {
            "Front" => Ok(Self::Front) ,
            "Back" => Ok(Self::Back),
            "Left" => Ok(Self::Left),
            "Right" => Ok(Self::Right),
            "Top" => Ok(Self::Top),
            "Bottom" => Ok(Self::Bottom),
            _ => Err("unknown face".into()),
        } 
    }
}

struct Cube {
    state : [Color; 54],
}

impl Cube {
    fn new () -> Self {
        Self::solved()
    }

    fn solved() -> Cube {
        let mut state = [Color::White; 54];
        for face in Face::VALUES.iter() {
            let offset = face.offset();
            for j in 0..9 {
                state[offset + j] = face.color();
            }
        }

        Cube { state }
    }

    fn from_file( path : &std::path::Path ) -> Result<Cube, Box<dyn Error>> {
       let json_str: String = std::fs::read_to_string(path)?;
       let parse: HashMap<String, Vec<String>> = serde_json::from_str(&json_str)?;
       let mut state: [Color; 54] = [Color::White; 54];
       for (face, values) in parse.iter() {
        let offset = Face::from_str(face)?.offset();
        for j in 0..9 {
            state[offset + j] = Color::from_str(&values[j])?;
        } 
       }

       Ok( Cube { state } )
    } 

}