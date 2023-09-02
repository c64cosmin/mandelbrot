extern crate termion;

use std::ops;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

struct Complex{
    x: f32,
    y: f32,
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
    fn len(&self) -> f32{
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
    width: i32,
    height: i32,
}

impl Bitmap{
    fn new(width: i32, height: i32) -> Bitmap{
        let bitmap : Vec<i32> = vec![0; (width*height) as usize];

        Bitmap{
            bitmap,
            width,
            height,
        }
    }

    fn display(&self){
        for line in self.bitmap.chunks(self.width as usize){
            for pixel in line.iter(){
                if pixel % 3 == 1{
                    print!("x")
                }else{
                    print!(" ")
                }
            }
            println!("");
        }
    }

    fn fill_circle(&mut self){
        for i in 0..self.height {
            for j in 0..self.width {
                let x: i32 = j - self.width / 2;
                let y: i32 = i - self.height / 2;
                let g: i32 = ((x*x+y*y) as f32).sqrt() as i32;
                self.set(j as i32, i as i32, g);
            }
        }
    }

    fn fill_madelbrot(&mut self, x_pos: i32, y_pos: i32){
        for i in 0..self.height {
            for j in 0..self.width {
                let x: f32 = (j - self.width / 2 + x_pos) as f32 / self.width as f32 - 0.3;
                let y: f32 = (i - self.height / 2 + y_pos) as f32 / self.height as f32;
                let zoom: f32 = 2.0;
                let mut z = Complex{x: 0.0, y: 0.0};
                let c = Complex{x: x*zoom, y: y*zoom};
                for _it in 0..1000{
                    z = z.sq().add(&c)
                }

                let mut g = 1;
                if f32::is_nan(z.len()) {
                    g = 0;
                }

                self.set(j as i32, i as i32, g);
            }
        }
    }

    fn set(&mut self, x:i32, y:i32, v: i32){
        let i = y*self.width + x;
        self.bitmap[i as usize] = v;
    }
}

fn main() {
    let mut bitmap: Bitmap = Bitmap::new(70, 40);

    bitmap.fill_circle();
    bitmap.display();

    println!("");

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut pos_x = 0;
    let mut pos_y = 0;

    for event in stdin.keys(){
        match event{
            Ok(key) => match key{
                Key::Up => pos_y -= 1,
                Key::Down => pos_y += 1,
                Key::Left => pos_x -= 1,
                Key::Right => pos_x += 1,
                Key::Ctrl('c') => break,
                Key::Char('q') => break,
                _ => {}
            },
            _ => break,
        }

        bitmap.fill_madelbrot(pos_x, pos_y);
        bitmap.display();

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
}
