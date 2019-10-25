#![feature(prelude_import)]
#![feature(print_internals)]
#![feature(fmt_internals)]
#![allow(deprecated)]
extern crate std;
extern crate alloc;

use std::fmt::*;
use termion::color;

pub struct Diff<T>(T, T);

pub fn diff<T>(a: T, b: T) -> Diff<T> {
    Diff(a, b)
}

fn clone_args(f: &Formatter) -> [std::fmt::rt::v1::Argument; 1] {
    [::core::fmt::rt::v1::Argument {
        position: ::core::fmt::rt::v1::Position::At(0usize),
        format: ::core::fmt::rt::v1::FormatSpec {
            fill: f.fill(),
            align: match f.align() {
                Some(a) => match a {
                    Alignment::Left => ::core::fmt::rt::v1::Alignment::Left,
                    Alignment::Right => ::core::fmt::rt::v1::Alignment::Right,
                    Alignment::Center => ::core::fmt::rt::v1::Alignment::Center,
                },
                None => ::core::fmt::rt::v1::Alignment::Unknown
            },
            flags: f.flags(),
            precision: match f.precision() {
                Some(p) => ::core::fmt::rt::v1::Count::Is(p),
                None => ::core::fmt::rt::v1::Count::Implied,
            },
            width: match f.width() {
                Some(w) => ::core::fmt::rt::v1::Count::Is(w),
                None => ::core::fmt::rt::v1::Count::Implied,
            },
        },
    }]
}

fn write_diff(one: String, two: String, f:&mut Formatter) -> Result {
    if one.len() != two.len() {
        panic!("Diff requires that the formmated versions are the same length\n \
                Try adding a width paramter to the format string");
    }
    let mut normal = true;
    write!(f, "{}", color::Fg(color::Green))?;
    for c in one.chars().zip(two.chars()) {
        if c.0 == c.1 {
            if normal {
                normal = false;
                write!(f, "{}", color::Fg(color::Green))?;
            }
        } else {
            if !normal {
                normal = true;
                write!(f, "{}", color::Fg(color::Red))?;
            }
        }
        write!(f, "{}", c.0)?;
    }
    write!(f, "{}", color::Fg(color::Reset))
}

impl<T: Display> Display for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Display::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Display::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: Binary> Binary for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Binary::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Binary::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: LowerHex> LowerHex for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::LowerHex::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::LowerHex::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: UpperHex> UpperHex for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::UpperHex::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::UpperHex::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: Octal> Octal for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Octal::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Octal::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: UpperExp> UpperExp for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::UpperExp::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::UpperExp::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: LowerExp> LowerExp for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::LowerExp::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::LowerExp::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}
impl<T: Pointer> Pointer for Diff<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Pointer::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Pointer::fmt,
            )],
            &cp_args,
        ));
        write_diff(one, two, f)
    }
}

pub struct Mask<T>(T, T);

pub fn mask<T>(a: T, b: T) -> Mask<T> {
    Mask(a, b)
}

fn write_mask(one: String, two: String, f:&mut Formatter) -> Result {
    if one.len() != two.len() {
        panic!("Diff requires that the formmated versions are the same length\n \
                Try adding a width paramter to the format string");
    }
    let mut normal = true;
    write!(f, "{}", color::Fg(color::Green))?;
    for c in one.chars().zip(two.chars()) {
        if c.1 == '0' {
            if normal {
                normal = false;
                write!(f, "{}", color::Fg(color::Green))?;
            }
        } else {
            if !normal {
                normal = true;
                write!(f, "{}", color::Fg(color::Red))?;
            }
        }
        write!(f, "{}", c.0)?;
    }
    write!(f, "{}", color::Fg(color::Reset))
}

impl<T: Display> Display for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Display::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Display::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: Binary> Binary for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Binary::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Binary::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: LowerHex> LowerHex for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::LowerHex::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::LowerHex::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: UpperHex> UpperHex for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::UpperHex::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::UpperHex::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: Octal> Octal for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Octal::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Octal::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: UpperExp> UpperExp for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::UpperExp::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::UpperExp::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: LowerExp> LowerExp for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::LowerExp::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::LowerExp::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}
impl<T: Pointer> Pointer for Mask<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cp_args = clone_args(f);
        let one = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.0,
                ::core::fmt::Pointer::fmt,
            )],
            &cp_args,
        ));
        let two = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &[::core::fmt::ArgumentV1::new(
                &self.1,
                ::core::fmt::Pointer::fmt,
            )],
            &cp_args,
        ));
        write_mask(one, two, f)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn format_expanded() {
        let value = 4;
        let tmp: String = alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
            &[""],
            &match (&value,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
            &[::core::fmt::rt::v1::Argument {
                position: ::core::fmt::rt::v1::Position::At(0usize),
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 8u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Is(2usize),
                },
            }],
        ));
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &[::core::fmt::ArgumentV1::new(
                &tmp,
                ::core::fmt::Display::fmt,
            )],
            &[::core::fmt::rt::v1::Argument {
                position: ::core::fmt::rt::v1::Position::At(0usize),
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 8u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Is(2usize),
                },
            }],
        ));
    }
}