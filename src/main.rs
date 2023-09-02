extern crate termion;

use std::thread;
use std::time::Duration;
use std::ops;
use termion::async_stdin;
use termion::event::Key;
use termion::terminal_size;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::color;
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
    mode: i32,
}

fn fill_color(intensity: i32){
    let color = 8;
    let chars = 5;
    match (intensity%(color*chars))/chars{
        0 => print!("{}{}", color::Bg(color::Black)     , color::Fg(color::Blue)      ,),
        1 => print!("{}{}", color::Bg(color::Blue)      , color::Fg(color::LightBlue) ,),
        2 => print!("{}{}", color::Bg(color::LightBlue) , color::Fg(color::LightCyan) ,),
        3 => print!("{}{}", color::Bg(color::LightCyan) , color::Fg(color::LightWhite),),
        4 => print!("{}{}", color::Bg(color::LightWhite), color::Fg(color::LightCyan) ,),
        5 => print!("{}{}", color::Bg(color::LightCyan) , color::Fg(color::LightBlue) ,),
        6 => print!("{}{}", color::Bg(color::LightBlue) , color::Fg(color::Blue)      ,),
        7 => print!("{}{}", color::Bg(color::Blue)      , color::Fg(color::Black)     ,),
        _ => {},
    }

    match (intensity%(color*chars))%chars{
        /*
        0 => print!("{}", "."),
        1 => print!("{}", "x"),
        2 => print!("{}", "%"),
        3 => print!("{}", "#"),
        4 => print!("{}", "@"),
        */
        0 => print!(" "),
        1 => print!("{}", '\u{2591}'),
        2 => print!("{}", '\u{2592}'),
        3 => print!("{}", '\u{2593}'),
        4 => print!("{}", '\u{2593}'),
        _ => {},
    }
}

impl Bitmap{
    fn new(width: i32, height: i32) -> Bitmap{
        let bitmap : Vec<i32> = vec![0; (width*height) as usize];

        Bitmap{
            bitmap,
            w: width,
            h: height,
            mode: 1,
        }
    }

    fn display(&self){
        match self.mode{
            1 => self.display_color(),
            _ => self.display_outline(),
        }
    }

    fn set_mode(&mut self, mode: i32){
        self.mode = mode;
    }

    fn display_outline(&self){
        let mut it = 0;
        for line in self.bitmap.chunks(self.w as usize){
            if it != 0{
                print!("\n");
            }

            for pixel in line.iter(){
                match pixel{
                    -1 => print!("{}{} ", color::Fg(color::White), color::Bg(color::Black)),
                    _ => print!("{}", '\u{2593}'),
                }
            }

            print!("\r");
            it += 1;
        }
    }

    fn display_color(&self){
        let mut it = 0;
        for line in self.bitmap.chunks(self.w as usize){
            if it != 0{
                print!("\n");
            }

            for pixel in line.iter(){
                match pixel{
                    -1 => print!("{}{} ", color::Fg(color::White), color::Bg(color::Black)),
                    _ => fill_color(*pixel),
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

    fn fill_madelbrot(&mut self, pos: &Position, complexity: &i32){
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
                for _it in 0..complexity*100{
                    z = z.sq().add(&c)
                }

                let mut g = (z.len()*100.0) as i32;
                if f64::is_nan(z.len()) {
                    g = -1;
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

fn display_all(stdout: &mut RawTerminal<std::io::Stdout>, bitmap: &mut Bitmap, help: &bool, pos: &Position, complexity: &i32){
    print!("{}", termion::cursor::Goto(1,1));
    stdout.flush().unwrap();

    bitmap.fill_madelbrot(pos, complexity);
    bitmap.display();

    if *help{
        print!("{}", termion::cursor::Goto(1,1));
        print!("{}{} ", color::Fg(color::White), color::Bg(color::Black));
        print!("pos:{},{}-{}\r\n", pos.x, pos.y, pos.z);
        print!("? - Open this message\r\n");
        print!("Arrows- Move Slower\r\n");
        print!("wasd  - Move\r\n");
        print!("WASD  - Move Faster\r\n");
        print!("+-    - Zoom\r\n");
        print!("()    - Depth {}\r\n", complexity);
        print!("1     - Color mode\r\n");
        print!("2     - Outline mode\r\n");
        print!("q     - Quit\r\n");
    }

    stdout.flush().unwrap();
}

fn start() {
    let term_size = terminal_size().unwrap();
    let mut bitmap: Bitmap = Bitmap::new(term_size.0 as i32, term_size.1 as i32);

    //bitmap.fill_circle();
    //bitmap.display();

    println!("");

    let stdin = stdin();
    let mut stdout: RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    stdout.activate_raw_mode().unwrap();

    let mut help = true;
    let mut pos: Position = Position { x: 0.0, y: 0.0, z: 0.0 };
    let mut complexity: i32 = 7;

    display_all(&mut stdout, &mut bitmap, &help, &pos, &complexity);

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
                Key::Char(')')=> complexity += 1,
                Key::Char('(')=> complexity -= 1,
                Key::Char('1')=> bitmap.set_mode(1),
                Key::Char('2')=> bitmap.set_mode(2),
                Key::Char('+')=> pos.z += 0.2,
                Key::Char('?')=> help = !help,
                Key::Ctrl('c') => break,
                Key::Char('q') => break,
                _ => {}
            },
            _ => break,
        }

        display_all(&mut stdout, &mut bitmap, &help, &pos, &complexity);
    }

    stdout.suspend_raw_mode().unwrap();
}

fn main_async(){
    let mut stdin = async_stdin().keys();
    let mut stdout: RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    loop {
        match stdin.next() {
            Some(result) => match result{
                Ok(key) => match key{
                    Key::Char('q') => {
                        print!("Quitting");
                        break;
                    },
                    key => {
                        print!("Key pressed: {:?}", key);
                    }
                },
                _ => {},
            },
            _ => {}
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(20));
    }

    stdout.suspend_raw_mode().unwrap();
}

fn main(){
    start();
}
