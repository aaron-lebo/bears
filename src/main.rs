use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate glium;

fn load_shader(path: &str) -> String {
    let mut file = File::open(format!("src/shaders/{}", path)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

type Matrix = [[f32; 4]; 4]; 

fn model(s: f32, dx: f32, dy: f32, dz: f32) -> Matrix {        
    [
        [s,   0.0, 0.0, 0.0],
        [0.0, s,   0.0, 0.0],
        [0.0, 0.0, s,   0.0],
        [dx,  dy,  dz,  1.0],
    ]
}

type Pos = [f32; 3]; 

fn normalize(pos: Pos) -> Pos {        
    let len: f32 = pos.iter().map(|x| x * x).sum();
    let len = len.sqrt();
    [pos[0] / len, pos[1] / len, pos[2] / len]
}

fn cross(a: Pos, b: Pos) -> Pos {        
    [
        b[1] * a[2] - b[2] * a[1],
        b[2] * a[0] - b[0] * a[2],
        b[0] * a[1] - b[1] * a[0],
    ] 
}

fn dot(a: Pos, b: Pos) -> f32 {        
     -b[0] * a[0] - b[1] * a[1] - b[2] * a[2]
}
 
fn look(pos: Pos, target: Pos, up: Pos) -> Matrix {        
    let f = normalize(target); 
    let s = normalize(cross(f, up));
    let u = cross(s, f);    
    [
        [s[0],        u[0],        f[0],        0.0],
        [s[1],        u[1],        f[1],        0.0],
        [s[2],        u[2],        f[2],        0.0],
        [dot(s, pos), dot(u, pos), dot(f, pos), 1.0],
    ]
}

fn projection(dims: (u32, u32)) -> Matrix {        
    let (w, h) = dims;
    let aspect = h as f32 / w as f32;

    let fov: f32 = 3.141592 / 3.0;
    let f = 1.0 / (fov / 2.0).tan();
    let (znear, zfar) = (0.1, 1024.0);
    let div = zfar - znear;

    [
        [f*aspect, 0.0, 0.0,                   0.0],
        [0.0,      f,   0.0,                   0.0],
        [0.0,      0.0, (zfar+znear)/div,      1.0],
        [0.0,      0.0, -(2.0*zfar*znear)/div, 0.0],
    ]
}

#[derive(Copy, Clone)]
struct Vertex {
    position: Pos,
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();

    let shape = vec![
    	Vertex{position: [-0.5,  0.5, -0.5]},
 	Vertex{position: [ 0.5,  0.5, -0.5]},
    	Vertex{position: [-0.5, -0.5, -0.5]},
    	Vertex{position: [ 0.5, -0.5, -0.5]}, 	    	
	Vertex{position: [-0.5,  0.5,  0.5]},
 	Vertex{position: [ 0.5,  0.5,  0.5]},
    	Vertex{position: [-0.5, -0.5,  0.5]},
    	Vertex{position: [ 0.5, -0.5,  0.5]}, 	
    ];
    let inds: [u16; 36] = [ 
        0, 1, 2, 
        2, 3, 1,         
	4, 5, 6, 
        6, 7, 5, 
	0, 1, 4,
	4, 5, 1,
	2, 3, 6,
	6, 7, 3,
        0, 4, 2,
        2, 6, 4,
	5, 1, 7,
	7, 3, 1u16,
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &inds).unwrap();
    let program = glium::Program::from_source(&display, &load_shader("vertex.glsl"), &load_shader("fragment.glsl"), None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.3, 0.3, 1.0), 1.0);

        let uniforms = uniform!{ 
            model:      model(0.05, 0.0, 0.0, 2.0),
            view:       look([2.0, -1.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]), 
            projection: projection(target.get_dimensions()),
        };
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
           },
           .. Default::default()
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
