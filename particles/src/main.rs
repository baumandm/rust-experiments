#![feature(macro_rules)]
#![feature(globs)]
#![feature(phase)]
#![allow(missing_copy_implementations)]

#[phase(plugin)]
extern crate gl_generator;
extern crate glfw;

extern crate time;

use glfw::{Glfw, Context, OpenGlProfileHint, WindowHint, WindowMode};

use std::rand::{task_rng, Rng};

use std::num::FloatMath;

use self::gl::Gl;


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod gl {
    use self::types::*;
    generate_gl_bindings! {
        api: "gl",
        profile: "core",
        version: "3.0",
        generator: "struct",
        extensions: [
            "GL_EXT_texture_filter_anisotropic",
        ]
    }
}

struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32
}

struct Particle {
    life: f32,
    fade: f32,
    x: f32,
    y: f32,
    xi: f32,
    yi: f32,
    color: Color
}

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static MAX_PARTICLES: uint = 10000;
static MAX_LIFE: f32 = 2.5;
static PARTICLE_SIZE: f32 = 1.5;


fn init_gl(gl: &Gl, glfw: &Glfw) {

    // Choose a GL profile that is compatible with OS X 10.7+
    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(WindowHint::OpenglProfile(OpenGlProfileHint::Core));

    glfw.set_swap_interval(1);

    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH.to_i32().unwrap(), WINDOW_HEIGHT.to_i32().unwrap());
        gl.MatrixMode(gl::PROJECTION);
        gl.LoadIdentity();
        gl.Ortho(0.0, 800.0, 600.0, 0.0, 0.0, 1.0);

        gl.Enable(gl::TEXTURE_2D);
        gl.Enable(gl::BLEND);
        gl.ShadeModel(gl::SMOOTH);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE);
    }
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (window, _) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "OpenGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    // Load the OpenGL function pointers
    let gl = Gl::load_with(|s| window.get_proc_address(s));

    init_gl(&gl, &glfw);

    let mut particles: Vec<Particle> = Vec::with_capacity(MAX_PARTICLES);

    while !window.should_close() {
        // Poll events
        glfw.poll_events();

        // Clear the screen to black
        unsafe {
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);

            gl.PointSize(PARTICLE_SIZE);
            gl.Begin(gl::POINTS);

            // Draw particles
            for p in particles.iter() {
                gl.Color4f(p.color.red, p.color.green, p.color.blue, p.color.alpha);
                gl.Vertex2f(p.x, p.y);
            }

            gl.End();
        }

        move_particles(&mut particles);

        // Swap buffers
        window.swap_buffers();

    }
}

fn create_new_particle() -> Particle {
    let angle: f32 = task_rng().gen_range(0.0, 360.0);
    let v: f32 = task_rng().gen_range(0.1, 1.0);

    let timespec = time::get_time();
    let millis = timespec.nsec as f32 / 1000.0 / 1000.0;
    let sec = timespec.sec.rem(100) as f32;

    let radius = (WINDOW_WIDTH as f32).min(WINDOW_HEIGHT as f32) / 4.0;

    let x = millis.sin() * radius;
    let y = millis.cos() * radius;

    Particle {
        life: task_rng().gen_range(MAX_LIFE * 0.8, MAX_LIFE),
        fade: task_rng().gen_range(0.01, 0.05),
        x: x + WINDOW_WIDTH as f32 / 2.0,
        y: y + WINDOW_HEIGHT as f32 / 2.0,
        xi: angle.cos() * v,
        yi: angle.sin() * v,
        color: Color {
            red: sec.sin(),
            green: sec.cos(),
            blue: task_rng().gen_range(0.4, 0.6),
            alpha: 1.0
        }
    }
}

fn move_particles(particles: &mut Vec<Particle>) {
    // Move & decay existing particles
    for mut particle in particles.iter_mut() {
        particle.life -= particle.fade;
        particle.x += particle.xi;
        particle.y += particle.yi;
        particle.xi *= 0.999;
        particle.yi *= 0.999;

        particle.color.alpha = 1.0 * particle.life / MAX_LIFE;
        particle.color.red += particle.fade / 3.0;
        particle.color.green += particle.fade / 3.0;
        particle.color.blue += particle.fade / 3.0;
    }

    // Replace dead particles
    for i in range(0u, particles.len()) {
        if particles[i].life < 0.05 {
            particles.remove(i);
            particles.insert(i, create_new_particle());
        }
    }

    // Add new particles if missing
    let mut i = 0i;
    loop {
        if particles.len() < MAX_PARTICLES {
            particles.push(create_new_particle());
            i += 1 ;
        } else {
            break;
        }

        if i > 20 {
            break;
        }
    }
}
