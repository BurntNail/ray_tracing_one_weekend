use nom::{
    bytes::complete::take,
    character::complete::{multispace0, multispace1, u16, u32},
    combinator::map,
    sequence::tuple,
    IResult,
};
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct PPMImage {
    pub width: u32,
    pub height: u32,
    pub max: u32,
    pub pixels: Vec<(u16, u16, u16)>,
}

impl<'a> TryFrom<&'a str> for PPMImage {
    type Error = nom::Err<nom::error::Error<String>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        PPMImage::parse(value)
            .map(|(_, img)| img)
            .map_err(|e| e.to_owned())
    }
}

impl PPMImage {
    pub fn to_normalised_rgba (&self) -> Vec<[f32; 4]> {
        let mut list = Vec::with_capacity((self.width * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                let (raw_red, raw_green, raw_blue) = self[(x as usize, y as usize)];

                let colour = [
                    raw_red as f32,
                    raw_green as f32,
                    raw_blue as f32,
                    self.max as f32,
                ]
                    .map(|col| col / self.max as f32);
                
                list.push(colour);
            }
        }
        
        list
    }
    
    
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, tag) = take(2_usize)(input)?;
        let (input, _) = multispace0(input)?;
        match tag {
            "P3" => Self::parse_p3(input),
            "P6" => Self::parse_p6(input),
            _ => panic!("bad magic bits {tag:?}"),
        }
    }

    fn parse_p3(input: &str) -> IResult<&str, Self> {
        let (input, (width, height, max)) = map(
            tuple((u32, multispace1, u32, multispace1, u32, multispace1)),
            |(w, _, h, _, max, _)| (w, h, max),
        )(input)?;

        fn parse_px(input: &str) -> IResult<&str, (u16, u16, u16)> {
            map(
                tuple((u16, multispace1, u16, multispace1, u16, multispace1)),
                |(r, _, g, _, b, _)| (r, g, b),
            )(input)
        }

        let mut pixels = Vec::with_capacity((width * height) as usize);
        let mut input = input;
        for _ in 0..(width * height) {
            let (local_input, rgb) = parse_px(input)?;
            pixels.push(rgb);
            input = local_input;
        }

        Ok((
            input,
            Self {
                width,
                height,
                max,
                pixels,
            },
        ))
    }
    fn parse_p6(input: &str) -> IResult<&str, Self> {
        todo!("Parse P6")
    }
}

impl Index<(usize, usize)> for PPMImage {
    type Output = (u16, u16, u16);

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[y * (self.width as usize) + x]
    }
}
