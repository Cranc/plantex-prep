use glium::{self, DisplayBuild, glutin};
use glium::backend::glutin_backend::GlutinFacade;
use super::Config;
use render::Renderer;
use event_manager::{EventManager, EventResponse};

pub fn run(config: &Config) -> Result<(), ()> {
    let context = try!(create_context(config));
    let renderer = Renderer::new(context.clone());
    let event_manager = EventManager::new(context.clone());

    loop {
        renderer.render().expect("error while rendering");

        let event_resp = event_manager.poll_events();
        if event_resp == EventResponse::Quit {
            break;
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
        .with_dimensions(config.resolution.width, config.resolution.height)
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
