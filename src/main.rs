///     Documentación interna
/// 
///     Autor: Oli Viau 23544
///     
///     Modificaciones:
///     [000] 15/07/2025
///     [001] 18/07/2025
/// 
///     Objetivo: Crear el "Conway's Game of Life" utilizando las reglas planteadas
/// 
///     Recursos:
///         ChatGPT: consultas generales y ayuda con la creación del GIF
///         https://www.youtube.com/watch?v=kfcjJeNo_EA
///         https://www.youtube.com/watch?v=jhhco5E5Yq4
///         https://dev.to/dineshgdk/game-of-life-in-rust-4mfc
///         https://crates.io/crates/conways_game_of_life_lib_rust/versions
 


use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufWriter;
use gif::{Frame, Encoder, Repeat}; //GIF

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: usize = 8;
const GIF_SCALE: u16 = 2;
const GIF_FRAME_SKIP: usize = 3;
//COLORES
const BACKGROUND_COLOR: u32 = 0xFFC0CB; 
const CELL_COLOR: u32 = 0x13F1ED;       
const DEAD_COLOR: u32 = 0xFFC0CB;       

//Main
fn main() {
    let window_width = WIDTH * CELL_SIZE;
    let window_height = HEIGHT * CELL_SIZE;
    //Ventana
    let mut window = Window::new(
        "Conway's Game of Life - ESC to exit",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Crear el GIF
    let file = File::create("conway_life.gif").expect("No se pudo crear el archivo GIF"); //Si no encuentra el path da error
    let mut encoder = Encoder::new(
        BufWriter::new(file), 
        (WIDTH / GIF_SCALE as usize) as u16, 
        (HEIGHT / GIF_SCALE as usize) as u16, 
        &[]
    ).unwrap();
    
    encoder.set_repeat(Repeat::Infinite).unwrap();
    //Colores de las celulas
    let mut game_buffer = vec![DEAD_COLOR; WIDTH * HEIGHT];
    let mut display_buffer = vec![BACKGROUND_COLOR; window_width * window_height];
    //Crea at random el juego
    initialize_random(&mut game_buffer);
    
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(100);
    let mut frame_count = 0;
    //Si se presiona escape sale y mientras la ventana exista va a ir actualizando
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed() >= update_interval {
            update_game(&mut game_buffer);
            last_update = Instant::now();
            
            if frame_count % GIF_FRAME_SKIP == 0 {
                add_to_gif(&game_buffer, &mut encoder);
            }
            frame_count += 1;
        }
        
        render(&game_buffer, &mut display_buffer, window_width);
        window.update_with_buffer(&display_buffer, window_width, window_height).unwrap();
    }
    //Revisar si si jala
    println!("GIF generado como 'conway_life.gif'");
}

fn initialize_random(buffer: &mut [u32]) {
    for pixel in buffer.iter_mut() {
        // Da una posibilidad de que las celdas esten vivas at random
        *pixel = if rand::random::<f32>() < 0.15 { CELL_COLOR } else { DEAD_COLOR };
    }
}
//Actualiza el juego
fn update_game(buffer: &mut [u32]) {
    let mut new_buffer = buffer.to_vec();
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let alive = buffer[idx] == CELL_COLOR;
            let live_neighbors = count_live_neighbors(buffer, x, y);
            
            // Reglas del juego
            new_buffer[idx] = match (alive, live_neighbors) {
                (true, x) if x < 2 => DEAD_COLOR,    // Subpoblación
                (true, 2) | (true, 3) => CELL_COLOR, // Supervivencia
                (true, x) if x > 3 => DEAD_COLOR,    // Sobrepoblación
                (false, 3) => CELL_COLOR,           // Reproducción
                _ => buffer[idx],                   // Sin cambios
            };
        }
    }
    
    buffer.copy_from_slice(&new_buffer);
}

//Cuenta las celulas vivas en casillas adyacentes
fn count_live_neighbors(buffer: &[u32], x: usize, y: usize) -> u8 {
    let mut count = 0;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            
            let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;
            
            if buffer[ny * WIDTH + nx] == CELL_COLOR {
                count += 1;
            }
        }
    }
    
    count
}
//Renderizar
fn render(game_buffer: &[u32], display_buffer: &mut [u32], display_width: usize) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cell_color = game_buffer[y * WIDTH + x];
            
            for dy in 0..CELL_SIZE {
                for dx in 0..CELL_SIZE {
                    let display_y = y * CELL_SIZE + dy;
                    let display_x = x * CELL_SIZE + dx;
                    display_buffer[display_y * display_width + display_x] = cell_color;
                }
            }
        }
    }
}
//Crea el archivo GIF
fn add_to_gif(game_buffer: &[u32], encoder: &mut Encoder<BufWriter<File>>) {
    let gif_width = (WIDTH / GIF_SCALE as usize) as usize;
    let gif_height = (HEIGHT / GIF_SCALE as usize) as usize;
    
    let mut frame_buffer = vec![0; gif_width * gif_height];
    
    for y in 0..gif_height {
        for x in 0..gif_width {
            let src_x = x * GIF_SCALE as usize;
            let src_y = y * GIF_SCALE as usize;
            let cell = game_buffer[src_y * WIDTH + src_x];
            frame_buffer[y * gif_width + x] = if cell == CELL_COLOR { 1 } else { 0 };
        }
    }
    
    let mut frame = Frame::default();
    frame.width = gif_width as u16;
    frame.height = gif_height as u16;
    frame.buffer = std::borrow::Cow::Borrowed(&frame_buffer);
    frame.delay = 5;
    
    frame.palette = Some(vec![
        0xFF, 0xC0, 0xCB, 
        0x13, 0xF1, 0xED,  
    ]);
    
    encoder.write_frame(&frame).unwrap();
}