use glium::{self, DisplayBuild, Surface, glutin};
use glium::backend::glutin_backend::GlutinFacade;
use super::Config;

pub fn run(config: &Config) -> Result<(), ()> {
    let context = try!(create_context(config));

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&context, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        out vec2 my_attr;      // our new attribute
        uniform mat4 matrix;
        void main() {
            my_attr = position;     // we need to set the value of each `out` variable.
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 my_attr;
        out vec4 color;
        void main() {
            color = vec4(my_attr, 0.0, 1.0);   // we build a vec4 from a vec2 and two floats
        }
    "#;

    let program =
        glium::Program::from_source(&context, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t = -0.5;

    'main: loop {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = context.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        for ev in context.poll_events() {
            match ev {
                glium::glutin::Event::Closed => break 'main,
                _ => (),
            }
        }
    }


    Ok(())
}

/// Creates the OpenGL context and prints useful information about the
/// success or failure of said action.
fn create_context(config: &Config) -> Result<GlutinFacade, ()> {
    // Create glium context
    // TODO: handle fullscreen
    // TODO: OpenGL version/profile
    // TODO: vsync
    let context = glutin::WindowBuilder::new()
        .with_dimensions(config.resolution.x, config.resolution.y)
        .with_title(config.window_title.clone())
        .build_glium();

    match context {
        Err(e) => {
            // TODO: print more user friendly output
            error!("OpenGL context creation failed! Detailed error:");
            error!("{}", e);

            Err(())
        }
        Ok(context) => {
            info!("OpenGL context was successfully built");

            // print some information about the aquired OpenGL context
            let glium::Version(api, major, minor) = *context.get_opengl_version();
            info!("Version of context: {} {}.{}",
                  if api == glium::Api::Gl { "OpenGL" } else { "OpenGL ES" },
                  major,
                  minor);

            let glium::Version(api, major, minor) = context.get_supported_glsl_version();
            info!("Supported GLSL version: {} {}.{}",
                  if api == glium::Api::Gl { "GLSL" } else { "GLSL ES" },
                  major,
                  minor);

            if let Some(mem) = context.get_free_video_memory() {
                let (mem, unit) = match mem {
                    _ if mem < (2 << 10) => (mem, "B"),
                    _ if mem < (2 << 20) => (mem >> 10, "KB"),
                    _ if mem < (2 << 30) => (mem >> 20, "MB"),
                    _ => (mem >> 30, "GB"),
                };
                info!("Free GPU memory (estimated): {}{}", mem, unit);
            }

            Ok(context)
        }
    }
}
