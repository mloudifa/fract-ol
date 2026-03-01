use macroquad::{prelude::*};
use num_complex::Complex;


const  MIN_REAL:f32 = -2.0;
const MAX_REAL:f32 = 1.0;
const MIN_IMAG:f32 = -1.0;
const MAX_IMAG:f32 = 1.0;
const WIDTH:i32 = 600;
const  HIGHT:i32 = 600;
const MAX_ITER:u32 = 100;

    fn complex_number(px:i32,py:i32) ->Complex<f32>
    {
        let c_re = MIN_REAL + (px as f32 / WIDTH as f32) * (MAX_REAL - MIN_REAL);
        let c_img = MIN_IMAG + (py as f32 / HIGHT as f32) * (MAX_IMAG - MIN_IMAG);

        
        let c = Complex::new(c_re, c_img);
        c
    }



fn map_color_to_iter(iter:u32,max_iter:u32)->Color
{
    if iter == max_iter{
        return BLACK;
    }

    let t = iter as f32 / max_iter as f32;

    Color::new(t, t, t, 1.0)
}

fn julia(a:f32,b:f32)
{

    let c = Complex::new(a, b);
      for x in 0..WIDTH {
        for y in 0..HIGHT
        {
                
                let mut z = complex_number(x, y);
                let mut iter:u32 = 0;

                while z.norm_sqr()<= 4.0 && iter < MAX_ITER {
                    z = z * z + c;
                    iter += 1;
                }   

                let col = map_color_to_iter(iter, MAX_ITER);
                draw_rectangle(x as f32, y as f32, 1.0, 1.0, col);
        }
    }
}

fn Mandelbrot()
{

    for x in 0..WIDTH {
        for y in 0..HIGHT
        {
                let c = complex_number(x, y);
                let mut z = Complex::new(0.0, 0.0);
                let mut iter:u32 = 0;

                while z.norm() <= 2.0 && iter < MAX_ITER {
                    z = z * z + c;
                    iter += 1;
                }   

                let col = map_color_to_iter(iter, MAX_ITER);
                draw_rectangle(x as f32, y as f32, 1.0, 1.0, col);
        }
    }
        
}


fn windo_conf() -> Conf
{
    Conf
    {
        window_title: "fract-ol".to_owned(),
        window_width: WIDTH,
        window_height:HIGHT,
        ..Default::default()
    }
}
#[macroquad::main(windo_conf)]

async fn main() 
{

     complex_number(400, 100);

        loop
        {
            clear_background(BLACK);
                julia(  0.355, 0.355);
            next_frame().await;
        } 
}