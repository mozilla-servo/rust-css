use float::round;
use libc::types::os::arch::c95::c_double;
use css_colors::*;
use cmp::Eq;

pub struct Color { red : u8, green : u8, blue : u8, alpha : float}

impl Color : Eq {
    pure fn eq(other: &Color) -> bool {
        return self.red == other.red && self.green == other.green && self.blue == other.blue &&
               self.alpha == other.alpha;
    }
    pure fn ne(other: &Color) -> bool {
        !self.eq(other)
    }
}

pub fn rgba(r : u8, g : u8, b : u8, a : float) -> Color {
    Color { red : r, green : g, blue : b, alpha : a}
}

pub fn rgb(r : u8, g : u8, b : u8) -> Color {
    return rgba(r, g, b, 1.0);
}

pub fn hsla(h : float, s : float, l : float, a : float) -> Color {
    // Algorithm for converting hsl to rbg taken from
    // http://www.w3.org/TR/2003/CR-css3-color-20030514/#hsl-color
    let m2 = if l <= 0.5 { l*(s + 1.0) } else { l + s - l*s };
    let m1 = l*2.0 - m2;
    let h = h / 360.0; 
    
    fn hue_to_rgb(m1 : float, m2 : float, h : float) -> float {
        let h = if h < 0.0 { h + 1.0 } else if h > 1.0 { h - 1.0 } else { h };

        match h {
          0.0 .. 1.0/6.0 => m1 + (m2 - m1)*h*6.0,
          1.0/6.0 .. 1.0/2.0 => m2,
          1.0/2.0 .. 2.0/3.0 => m1 + (m2 - m1)*(4.0 - 6.0*h),
          2.0/3.0 .. 1.0 => return m1,
          _ => fail ~"unexpected hue value"
        }
    }

    let r = round(255.0*hue_to_rgb(m1, m2, h + 1.0/3.0) as c_double);;
    let g = round(255.0*hue_to_rgb(m1, m2, h) as c_double);
    let b = round(255.0*hue_to_rgb(m1, m2, h - 1.0/3.0) as c_double);

    return rgba(r as u8, g as u8, b as u8, a);
}

pub fn hsl(h : float, s : float, l : float) -> Color {
    return hsla(h, s, l, 1.0);
}

impl Color {
    fn print() -> ~str {
        fmt!("rgba(%u,%u,%u,%f)", self.red as uint, self.green as uint,
             self.blue as uint, self.alpha)
    }
}

pub mod parsing {
    export parse_color;

    fn fail_unrecognized(col : &str) -> Option<Color> {
        warn!("Unrecognized color %s", col);
        return None;
    }

    /** Match an exact color keyword. */
    fn parse_by_name(color : &str) -> Option<Color> {
        let col = match color.to_lower() {
          ~"black" => black(),
          ~"silver" => silver(),
          ~"gray" => gray(),
          ~"grey" => gray(),
          ~"white" => white(),
          ~"maroon" => maroon(),
          ~"red" => red(),
          ~"purple" => purple(),
          ~"fuchsia" => fuchsia(),
          ~"green" => green(),
          ~"lime" => lime(),
          ~"olive" => olive(),
          ~"yellow" => yellow(),
          ~"navy" => navy(),
          ~"blue" => blue(),
          ~"teal" => teal(),
          ~"aqua" => aqua(),
          _ => return fail_unrecognized(color)
        };

        return Some(col);
    }
    
    /** Parses a color specification in the form rgb(foo,bar,baz) */
    fn parse_rgb(color : &str) -> Option<Color> {
        // Shave off the rgb( and the )
        let only_colors = color.substr(4u, color.len() - 5u);

        // split up r, g, and b
        let cols = only_colors.split_char(',');
        if cols.len() != 3u { return fail_unrecognized(color); }

        match (u8::from_str(cols[0]), u8::from_str(cols[1]), 
             u8::from_str(cols[2])) {
          (Some(r), Some(g), Some(b)) => { Some(rgb(r, g, b)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form rgba(foo,bar,baz,qux) */
    fn parse_rgba(color : &str) -> Option<Color> {
        // Shave off the rgba( and the )
        let only_vals = color.substr(5u, color.len() - 6u);

        // split up r, g, and b
        let cols = only_vals.split_char(',');
        if cols.len() != 4u { return fail_unrecognized(color); }

        match (u8::from_str(cols[0]), u8::from_str(cols[1]), 
             u8::from_str(cols[2]), float::from_str(cols[3])) {
          (Some(r), Some(g), Some(b), Some(a)) => { Some(rgba(r, g, b, a)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form hsl(foo,bar,baz) */
    fn parse_hsl(color : &str) -> Option<Color> {
        // Shave off the hsl( and the )
        let only_vals = color.substr(4u, color.len() - 5u);

        // split up h, s, and l
        let vals = only_vals.split_char(',');
        if vals.len() != 3u { return fail_unrecognized(color); }

        match (float::from_str(vals[0]), float::from_str(vals[1]), 
             float::from_str(vals[2])) {
          (Some(h), Some(s), Some(l)) => { Some(hsl(h, s, l)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form hsla(foo,bar,baz,qux) */
    fn parse_hsla(color : &str) -> Option<Color> {
        // Shave off the hsla( and the )
        let only_vals = color.substr(5u, color.len() - 6u);

        let vals = only_vals.split_char(',');
        if vals.len() != 4u { return fail_unrecognized(color); }

        match (float::from_str(vals[0]), float::from_str(vals[1]), 
             float::from_str(vals[2]), float::from_str(vals[3])) {
          (Some(h), Some(s), Some(l), Some(a)) => { Some(hsla(h, s, l, a)) }
          _ => { fail_unrecognized(color) }
        }
    }

    // Currently colors are supported in rgb(a,b,c) form and also by
    // keywords for several common colors.
    // TODO: extend this
    fn parse_color(color : &str) -> Option<Color> {
        match color {
          c if c.starts_with("rgb(") => parse_rgb(c),
          c if c.starts_with("rgba(") => parse_rgba(c),
          c if c.starts_with("hsl(") => parse_hsl(c),
          c if c.starts_with("hsla(") => parse_hsla(c),
          c => parse_by_name(c)
        }
    }
}

#[cfg(test)]
mod test {
    use css_colors::*;
    use option::unwrap;
    use parsing::parse_color;

    #[test]
    fn test_parse_by_name() {
        assert red().eq(&unwrap(parse_color(~"red")));
        assert lime().eq(&unwrap(parse_color(~"Lime")));
        assert blue().eq(&unwrap(parse_color(~"BLUE")));
        assert green().eq(&unwrap(parse_color(~"GreEN")));
        assert white().eq(&unwrap(parse_color(~"white")));
        assert black().eq(&unwrap(parse_color(~"Black")));
        assert gray().eq(&unwrap(parse_color(~"Gray")));
        assert silver().eq(&unwrap(parse_color(~"SiLvEr")));
        assert maroon().eq(&unwrap(parse_color(~"maroon")));
        assert purple().eq(&unwrap(parse_color(~"PURPLE")));
        assert fuchsia().eq(&unwrap(parse_color(~"FUCHSIA")));
        assert olive().eq(&unwrap(parse_color(~"oLiVe")));
        assert yellow().eq(&unwrap(parse_color(~"yellow")));
        assert navy().eq(&unwrap(parse_color(~"NAVY")));
        assert teal().eq(&unwrap(parse_color(~"Teal")));
        assert aqua().eq(&unwrap(parse_color(~"Aqua")));
        assert None == parse_color(~"foobarbaz");
    }

    #[test]
    fn test_parsing_rgb() {
        assert red().eq(&unwrap(parse_color(~"rgb(255,0,0)")));
        assert red().eq(&unwrap(parse_color(~"rgba(255,0,0,1.0)")));
        assert red().eq(&unwrap(parse_color(~"rgba(255,0,0,1)")));
        assert lime().eq(&unwrap(parse_color(~"rgba(0,255,0,1.00)")));
        assert rgb(1u8,2u8,3u8).eq(&unwrap(parse_color(~"rgb(1,2,03)")));
        assert rgba(15u8,250u8,3u8,0.5).eq(&unwrap(parse_color(~"rgba(15,250,3,.5)")));
        assert rgba(15u8,250u8,3u8,0.5).eq(&unwrap(parse_color(~"rgba(15,250,3,0.5)")));
        assert None == parse_color(~"rbga(1,2,3)");
    }

    #[test]
    fn test_parsing_hsl() {
        assert red().eq(&unwrap(parse_color(~"hsl(0,1,.5)")));
        assert lime().eq(&unwrap(parse_color(~"hsl(120.0,1.0,.5)")));
        assert blue().eq(&unwrap(parse_color(~"hsl(240.0,1.0,.5)")));
        assert green().eq(&unwrap(parse_color(~"hsl(120.0,1.0,.25)")));
        assert white().eq(&unwrap(parse_color(~"hsl(1.0,1.,1.0)")));
        assert white().eq(&unwrap(parse_color(~"hsl(129.0,0.3,1.0)")));
        assert black().eq(&unwrap(parse_color(~"hsl(231.2,0.75,0.0)")));
        assert black().eq(&unwrap(parse_color(~"hsl(11.2,0.0,0.0)")));
        assert gray().eq(&unwrap(parse_color(~"hsl(0.0,0.0,0.5)")));
        assert maroon().eq(&unwrap(parse_color(~"hsl(0.0,1.0,0.25)")));
        assert purple().eq(&unwrap(parse_color(~"hsl(300.0,1.0,0.25)")));
        assert fuchsia().eq(&unwrap(parse_color(~"hsl(300,1.0,0.5)")));
        assert olive().eq(&unwrap(parse_color(~"hsl(60.,1.0,0.25)")));
        assert yellow().eq(&unwrap(parse_color(~"hsl(60.,1.0,0.5)")));
        assert navy().eq(&unwrap(parse_color(~"hsl(240.0,1.0,.25)")));
        assert teal().eq(&unwrap(parse_color(~"hsl(180.0,1.0,.25)")));
        assert aqua().eq(&unwrap(parse_color(~"hsl(180.0,1.0,.5)")));
        assert None == parse_color(~"hsl(1,2,3,.4)");
    }
}


/** Define the colors specified by css */
pub mod css_colors {
    // The 16 basic css colors
    pub fn black() -> Color {
        Color {red : 0u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn silver() -> Color {
        Color {red : 192u8, green : 192u8, blue : 192u8, alpha : 1.0}
    }
    pub fn gray() -> Color {
        Color {red : 128u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn white() -> Color {
        Color {red : 255u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn maroon() -> Color {
        Color {red : 128u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn red() -> Color { 
        Color {red : 255u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn purple() -> Color {
        Color {red : 128u8, green : 0u8, blue : 128u8, alpha : 1.0}
    }
    pub fn fuchsia() -> Color {
        Color {red : 255u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn green() -> Color { 
        Color {red : 0u8, green : 128u8, blue : 0u8, alpha : 1.0}
    }
    pub fn lime() -> Color {
        Color {red : 0u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn olive() -> Color {
        Color {red : 128u8, green : 128u8, blue : 0u8, alpha : 1.0}
    }
    pub fn yellow() -> Color {
        Color {red : 255u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn navy() -> Color {
        Color {red : 0u8, green : 0u8, blue : 128u8, alpha : 1.0}
    }
    pub fn blue() -> Color {
        Color {red : 0u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn teal() -> Color {
        Color {red : 0u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn aqua() -> Color {
        Color {red : 0u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }


    // The other 130 css colors
    // TODO
}
