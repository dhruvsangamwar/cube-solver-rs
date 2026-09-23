#![allow(dead_code)]

use std::error::Error;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Color {
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

    pub fn from_str(str : &str) -> Result<Color, Box<dyn Error>>{
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


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Face {
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

    pub fn offset(self) -> usize {
        match self {
            Self::Front  => 0,
            Self::Back   => 9,
            Self::Left   => 18,
            Self::Right  => 27,
            Self::Top    => 36,
            Self::Bottom => 45
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::Front  => Color::Red,
            Self::Back   => Color::Orange,
            Self::Left   => Color::Green,
            Self::Right  => Color::Blue,
            Self::Top    => Color::White,
            Self::Bottom => Color::Yellow,
        }
    }

    pub fn from_str(str : &str) -> Result<Face, Box<dyn Error>>{
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    state : [Color; 54],
}

impl Cube {
    pub fn new () -> Self {
        Self::solved()
    }

    pub(crate) fn state_mut(&mut self) -> &mut [Color; 54] {
        &mut self.state
    }

    pub fn solved() -> Cube {
        let mut state = [Color::White; 54];
        for face in Face::VALUES.iter() {
            let offset = face.offset();
            for j in 0..9 {
                state[offset + j] = face.color();
            }
        }

        Cube { state }
    }

    pub fn show(&self) {
        let c = |i: usize| match self.state[i] {
            Color::Red    => 'R',
            Color::Orange => 'O',
            Color::Green  => 'G',
            Color::Blue   => 'B',
            Color::White  => 'W',
            Color::Yellow => 'Y',
        };

        // offsets
        let f = Face::Front.offset();
        let ba = Face::Back.offset();
        let l = Face::Left.offset();
        let r = Face::Right.offset();
        let t = Face::Top.offset();
        let bo = Face::Bottom.offset();

        println!("        Top");
        for row in 0..3 {
            println!("        {} {} {}", c(t + row*3), c(t + row*3+1), c(t + row*3+2));
        }
        println!("Left  Front  Right  Back");
        for row in 0..3 {
            println!("{} {} {}  {} {} {}  {} {} {}  {} {} {}",
                c(l + row*3), c(l + row*3+1), c(l + row*3+2),
                c(f + row*3), c(f + row*3+1), c(f + row*3+2),
                c(r + row*3), c(r + row*3+1), c(r + row*3+2),
                c(ba+ row*3), c(ba+ row*3+1), c(ba+ row*3+2),
            );
        }
        println!("        Bottom");
        for row in 0..3 {
            println!("        {} {} {}", c(bo + row*3), c(bo + row*3+1), c(bo + row*3+2));
        }
    }

    pub fn from_file( path : &std::path::Path ) -> Result<Cube, Box<dyn Error>> {
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