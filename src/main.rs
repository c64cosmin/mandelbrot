struct Bitmap{
    bitmap: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

impl Bitmap{
    fn new(w: usize, h: usize) -> Bitmap{
        let mut bitmap : Vec<Vec<i32>> = Vec::new();
        for _y in 0..h{
            let mut line : Vec<i32> = Vec::new();
            line.resize(w, 0);
            bitmap.push(line);
        }

        Bitmap{
            bitmap: bitmap,
            width: w as i32,
            height: h as i32,
        }
    }

    fn display(&self){
        for line in self.bitmap.iter(){
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
                let g: i32 = f32::sqrt((x*x+y*y) as f32) as i32;
                self.set(j as usize, i as usize, g);
            }
        }
    }

    fn set(&mut self, x:usize, y:usize, v: i32){
        self.bitmap[y][x] = v;
    }
}
fn main() {
    let mut bitmap: Bitmap = Bitmap::new(70, 40);

    bitmap.fill_circle();
    bitmap.display();

    println!("");
}
