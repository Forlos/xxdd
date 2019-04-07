extern crate ansi_term;
#[macro_use]
extern crate prettytable;
extern crate structopt;

use ansi_term::Color;
use structopt::StructOpt;

use std::io::Read;

#[derive(StructOpt, Debug)]
#[structopt(name = "xxdd", about = "xdd Deluxe - make a colorized hexdump")]
struct Opt {

    #[structopt(short = "A", help = "Print 0x41 characters with nice background")]
    print_0x41: bool,

}


fn main() {

    let opt = Opt::from_args();

    match run(opt) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }

}

fn run(opt: Opt) -> std::io::Result<()> {

    let mut stdin = std::io::stdin();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf)?;

    print_hexdump(opt ,&buf);

    Ok(())

}

fn print_hexdump(opt: Opt, buf: &[u8]) {
    use prettytable::Table;

    let mut table = Table::new();
    let format = prettytable::format::FormatBuilder::new()
        .padding(0, 1)
        .build();
    table.set_format(format);

    let chunks = buf.chunks(16);
    for (i, chunk) in chunks.enumerate() {

        let mut chunk_str = String::with_capacity(16);

        for (j,c) in chunk.iter().enumerate() {
            if j != 0 && j % 2 == 0 {
                chunk_str += " ";
            }
            if j != 0 && j % 8 == 0 {
                chunk_str += " ";
            }
            chunk_str += &format!("{:02X}", c);
        }

        table.add_row(row![
            format!("{:08X}:", i * 16),
            print_hex(&opt, chunk),
            print_hexstring(&opt, chunk),
        ]);

    }

    table.printstd();

}

fn print_hex(opt: &Opt, buf: &[u8]) -> String {

    let mut hex_str = String::with_capacity(50);

    for (i,b) in buf.iter().enumerate() {

        if i != 0 && i % 2 == 0 {
            hex_str += " ";
        }
        if i != 0 && i % 8 == 0 {
            hex_str += " ";
        }

        hex_str += &(match b {

            0x41 => {
                if opt.print_0x41 {
                    format!("{}", Color::Black.on(Color::Yellow).paint(format!("{:02X}", b)))
                }
                else {
                    format!("{}", Color::Blue.paint(format!("{:02X}", b)))
                }
            },

            0x00 => format!("{}", Color::Black.on(Color::Red).paint(format!("{:02X}", b))),
            0x04 => format!("{}", Color::Black.on(Color::Green).paint(format!("{:02X}", b))),
            0x09 => format!("{}", Color::Black.on(Color::Cyan).paint(format!("{:02X}", b))),
            0x0A => format!("{}", Color::Black.on(Color::Blue).paint(format!("{:02X}", b))),
            0x0D => format!("{}", Color::Black.on(Color::Purple).paint(format!("{:02X}", b))),

            0x30...0x39 => format!("{}", Color::Green.paint(format!("{:02X}", b))),
            0x41...0x5A | 0x61...0x7A => format!("{}", Color::Blue.paint(format!("{:02X}", b))),
            0x20...0x2F | 0x5B...0x60 | 0x7B...0x7E => format!("{}", Color::Red.paint(format!("{:02X}", b))),

            _ => format!("{:02X}", b),

        });

    }

    hex_str

}

fn print_hexstring(opt: &Opt, buf: &[u8]) -> String {

    let mut hex_str = String::with_capacity(16);
    hex_str += "|";

    for b in buf {

        hex_str += &(match b {

            0x41 => {
                if opt.print_0x41 {
                    format!("{}", Color::Black.on(Color::Yellow).paint((*b as char).to_string()))
                }
                else {
                    format!("{}", Color::Blue.paint((*b as char).to_string()))
                }
            },

            0x00 => format!("{}", Color::Black.on(Color::Red).paint(".")),
            0x04 => format!("{}", Color::Black.on(Color::Green).paint(".")),
            0x09 => format!("{}", Color::Black.on(Color::Cyan).paint(".")),
            0x0A => format!("{}", Color::Black.on(Color::Blue).paint(".")),
            0x0D => format!("{}", Color::Black.on(Color::Purple).paint(".")),

            0x30...0x39 => format!("{}", Color::Green.paint((*b as char).to_string())),
            0x41...0x5A | 0x61...0x7A => format!("{}", Color::Blue.paint((*b as char).to_string())),
            0x20...0x2F | 0x5B...0x60 | 0x7B...0x7E => format!("{}", Color::Red.paint((*b as char).to_string())),

            _ => format!("{}", "."),

        });

    }

    hex_str += "|";
    hex_str

}
