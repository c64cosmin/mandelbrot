extern crate termion;

use std::ops;
use termion::event::Key;
use termion::terminal_size;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use std::io::{Write, stdout, stdin};

struct Position{
    x: f64,
    y: f64,
    z: f64,
}

struct Complex{
    x: f64,
    y: f64,
}

impl ops::Add for Complex{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex{
        Complex{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul for Complex{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex{
        Complex{
            x: self.x*rhs.x - self.y*rhs.y,
            y: self.x*rhs.y + self.y*rhs.x,
        }
    }
}

impl Complex{
    fn len(&self) -> f64{
        (self.x*self.x+self.y*self.y).sqrt()
    }

    fn sq(&self) -> Complex{
        Complex{
            x: self.x*self.x - self.y*self.y,
            y: self.x*self.y + self.y*self.x,
        }
    }

    fn add(&self, rhs: &Complex) -> Complex{
        Complex{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Bitmap{
    bitmap: Vec<i32>,
    w: i32,
    h: i32,
}

impl Bitmap{
    fn new(width: i32, height: i32) -> Bitmap{
        let bitmap : Vec<i32> = vec![0; (width*height) as usize];

        Bitmap{
            bitmap,
            w: width,
            h: height,
        }
    }

    fn display(&self){
        let block = char::from_u32(0x2588).unwrap();
        let mut it = 0;
        for line in self.bitmap.chunks(self.w as usize){
            if it != 0{
                print!("\n");
            }

            for pixel in line.iter(){
                if pixel % 3 == 1{
                    print!("{}", block);
                }else{
                    print!(" ")
                }
            }
            print!("\r");
            it += 1;
        }
    }

    fn fill_circle(&mut self){
        for i in 0..self.h {
            for j in 0..self.w {
                let x: i32 = j - self.w / 2;
                let y: i32 = i - self.h / 2;
                let g: i32 = ((x*x+y*y) as f64).sqrt() as i32;
                self.set(j as i32, i as i32, g);
            }
        }
    }

    fn fill_madelbrot(&mut self, pos: &Position){
        for i in 0..self.h {
            for j in 0..self.w {
                let aspect_x = (self.w as f64 / self.h as f64) * 1.0;
                let aspect_y = 2.0;
                let zoom: f64 = f64::exp(-pos.z);
                let pixel_x: f64 = (j - self.w / 2) as f64 / self.w as f64;
                let pixel_y: f64 = (i - self.h / 2) as f64 / self.h as f64;
                let x: f64 = zoom * pixel_x + pos.x;
                let y: f64 = zoom * pixel_y + pos.y;
                let mut z = Complex{x: 0.0, y: 0.0};
                let c = Complex{
                    x: aspect_x * x,
                    y: aspect_y * y,
                };
                for _it in 0..1000{
                    z = z.sq().add(&c)
                }

                let mut g = 1;
                if f64::is_nan(z.len()) {
                    g = 0;
                }

                self.set(j as i32, i as i32, g);
            }
        }
    }

    fn set(&mut self, x:i32, y:i32, v: i32){
        let i = y*self.w + x;
        self.bitmap[i as usize] = v;
    }
}

fn display_all(stdout: &mut RawTerminal<std::io::Stdout>, bitmap: &mut Bitmap, help: &bool, pos: &Position){
    print!("{}", termion::cursor::Goto(1,1));
    stdout.flush().unwrap();

    bitmap.fill_madelbrot(pos);
    bitmap.display();

    if *help{
        print!("{}pos:{},{}-{}\r\n", termion::cursor::Goto(1,1), pos.x, pos.y, pos.z);
        print!("? - Open this message\r\n");
        print!("Arrows- Move Slower\r\n");
        print!("wasd  - Move\r\n");
        print!("WASD  - Move Faster\r\n");
        print!("+-    - Zoom\r\n");
        print!("q     - Quit\r\n");
    }

    stdout.flush().unwrap();
}

fn main() {
    let term_size = terminal_size().unwrap();
    let mut bitmap: Bitmap = Bitmap::new(term_size.0 as i32, term_size.1 as i32);

    bitmap.fill_circle();
    bitmap.display();

    println!("");

    let stdin = stdin();
    let mut stdout: RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    stdout.activate_raw_mode().unwrap();

    let mut help = true;
    let mut pos: Position = Position { x: 0.0, y: 0.0, z: 0.0 };

    for event in stdin.keys(){
        let zoom: f64 = f64::exp(-pos.z);

        match event{
            Ok(key) => match key{
                Key::Up => pos.y -= 0.01 * zoom,
                Key::Down => pos.y += 0.01 * zoom,
                Key::Left => pos.x -= 0.01 * zoom,
                Key::Right => pos.x += 0.01 * zoom,
                Key::Char('W')=> pos.y -= 1.0 * zoom,
                Key::Char('S')=> pos.y += 1.0 * zoom,
                Key::Char('A')=> pos.x -= 1.0 * zoom,
                Key::Char('D')=> pos.x += 1.0 * zoom,
                Key::Char('w')=> pos.y -= 0.1 * zoom,
                Key::Char('s')=> pos.y += 0.1 * zoom,
                Key::Char('a')=> pos.x -= 0.1 * zoom,
                Key::Char('d')=> pos.x += 0.1 * zoom,
                Key::Char('-')=> pos.z -= 0.2,
                Key::Char('+')=> pos.z += 0.2,
                Key::Char('?')=> help = !help,
                Key::Ctrl('c') => break,
                Key::Char('q') => break,
                _ => {}
            },
            _ => break,
        }

        display_all(&mut stdout, &mut bitmap, &help, &pos);
    }

    stdout.suspend_raw_mode().unwrap();
}
