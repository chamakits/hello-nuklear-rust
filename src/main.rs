#[macro_use]
extern crate nuklear;
extern crate nuklear_backend_gfx;

extern crate image;

extern crate gfx_core as core;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_device_gl as device_gl;

use nuklear::*;
use nuklear_backend_gfx::{Drawer, GfxBackend};

use gfx::Device as Gd;
use glutin::{GlContext, GlRequest};
use device_gl::Resources as R;
use core::{handle};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const MAX_VERTEX_MEMORY: usize = 512 * 1024;
const MAX_ELEMENT_MEMORY: usize = 128 * 1024;
const MAX_COMMANDS_MEMORY: usize = 64 * 1024;

pub type ColorFormatImpl = (gfx::format::R8_G8_B8_A8, gfx::format::Unorm);
pub type DepthFormatImpl = (gfx::format::D24_S8, gfx::format::Unorm);

fn create_base() -> (
        glutin::GlWindow, device_gl::Device, 
        device_gl::Factory, handle::RenderTargetView<R, ColorFormatImpl>, 
        handle::DepthStencilView<R, DepthFormatImpl>, glutin::EventsLoop)
{
    
    let gl_version = GlRequest::GlThenGles {
        opengles_version: (2, 0),
        opengl_version: (3, 3),
    };

    let builder = glutin::WindowBuilder::new().with_title("Nuklear Rust Gfx OpenGL Demo").with_dimensions(1280, 800);
    let context = glutin::ContextBuilder::new().with_gl(gl_version).with_vsync(true).with_srgb(false).with_depth_buffer(24);
    
    #[allow(unused_mut)]
    let mut event_loop = glutin::EventsLoop::new();
    let tup_return = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &event_loop);

    // return tup_return;
    return (tup_return.0,tup_return.1,tup_return.2,tup_return.3,tup_return.4, event_loop);
}

fn font_config_setup() -> nuklear::FontConfig {
    let mut cfg = FontConfig::with_size(0.0);
    cfg.set_oversample_h(3);
    cfg.set_oversample_v(2);
    cfg.set_glyph_range(font_cyrillic_glyph_ranges());
    cfg.set_ttf(include_bytes!("../res/fonts/Roboto-Regular.ttf"));

    return cfg;
}

fn convert_config_setup(null: &nuklear::DrawNullTexture) -> ConvertConfig {
    let mut config = ConvertConfig::default();
    config.set_null(null.clone());
    config.set_circle_segment_count(22);
    config.set_curve_segment_count(22);
    config.set_arc_segment_count(22);
    config.set_global_alpha(1.0f32);
    config.set_shape_aa(AntiAliasing::NK_ANTI_ALIASING_ON);
    config.set_line_aa(AntiAliasing::NK_ANTI_ALIASING_ON);
    return config;
}

fn set_font( cfg: &mut nuklear::FontConfig, atlas: &mut FontAtlas, size: f32, owned_by_atlas: bool) -> usize {
    cfg.set_ttf_data_owned_by_atlas(owned_by_atlas);
    cfg.set_size(size);
    return atlas.add_font_with_config(&cfg).unwrap();
}

fn main() {
    #[allow(unused_mut, unused_variables)]
    let (window, mut device, mut factory, main_color, mut main_depth, mut event_loop) = create_base();
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut cfg = font_config_setup();

    let mut allo = Allocator::new_vec();
    let mut drawer = Drawer::new(&mut factory, main_color, 36, MAX_VERTEX_MEMORY, MAX_ELEMENT_MEMORY, Buffer::with_size(&mut allo, MAX_COMMANDS_MEMORY), GfxBackend::OpenGlsl150);
    let mut atlas = FontAtlas::new(&mut allo);

    let font_14 = set_font(&mut cfg, &mut atlas, 14f32, false);

    let mut ctx = Context::new(&mut allo, atlas.font(font_14).unwrap().handle());


    let font_tex = {
        let (b, w, h) = atlas.bake(FontAtlasFormat::NK_FONT_ATLAS_RGBA32);
        drawer.add_texture(&mut factory, b, w, h)
    };
    
    let mut null = DrawNullTexture::default();
    atlas.end(font_tex, Some(&mut null));

    let mut config = convert_config_setup(&null);

    let mut closed = false;
    while !closed {
        ctx.input_begin();
        event_loop.poll_events(|event| {
            println!("{:?}", event);
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                }
            }
        });
        ctx.input_end();

        if closed {
            break;
        }

        let (fw, fh) = window.get_inner_size().unwrap();
        let scale = window.hidpi_factor();
        let scale = Vec2 { x: scale, y: scale };

        custom_simple_stuff(&mut ctx);
        encoder.clear(drawer.col.as_ref().unwrap(), [0.1f32, 0.2f32, 0.3f32, 1.0f32]);
        drawer.draw(&mut ctx, &mut config, &mut encoder, &mut factory, fw, fh, scale);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();

        ::std::thread::sleep(::std::time::Duration::from_millis(20));

        ctx.clear();
    }
}

fn custom_simple_stuff(ctx: &mut Context) {
    // ctx.style_set_font(media.font_atlas.font(media.font_20).unwrap().handle());
    ctx.begin(
        nk_string!("Basic Nuklear Rust!"),
        Rect { x: 320f32, y: 50f32, w: 275f32, h: 610f32 },
        PanelFlags::NK_WINDOW_BORDER as Flags | PanelFlags::NK_WINDOW_MOVABLE as Flags | PanelFlags::NK_WINDOW_TITLE as Flags,
    );
    ctx.layout_row_dynamic(30f32, 2);
    ctx.text("Free type:", TextAlignment::NK_TEXT_RIGHT as Flags);
    
    // ctx.style_set_font(media.font_atlas.font(media.font_14).unwrap().handle());
    ctx.end();
}
