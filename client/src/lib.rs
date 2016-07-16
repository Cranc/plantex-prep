extern crate base;
#[macro_use]
extern crate glium;

pub mod render;
mod config;

pub use config::Config;

pub fn start_game(config: Config) -> Result<(), ()> {
    let renderer = render::Renderer::with_config(&config);

    // Handle errors
    let renderer = match renderer {
        Err(e) => {
            // TODO: print more user friendly information
            println!("{}", e);
            return Err(());
        }
        Ok(renderer) => renderer,
    };

    renderer.run();

    Ok(())
}
