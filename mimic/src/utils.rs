use std::{cell::RefCell, ops::Range, rc::Rc};

use glium::Texture2d;
use imgui::TextureId;
use imgui_glium_renderer::Renderer;

pub fn register_texture(texture: Rc<Texture2d>, renderer: Rc<RefCell<Renderer>>) -> TextureId {
    renderer
        .borrow_mut()
        .textures()
        .insert(imgui_glium_renderer::Texture {
            texture: texture,
            sampler: glium::uniforms::SamplerBehavior {
                minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
                magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
                wrap_function: (
                    glium::uniforms::SamplerWrapFunction::BorderClamp,
                    glium::uniforms::SamplerWrapFunction::BorderClamp,
                    glium::uniforms::SamplerWrapFunction::BorderClamp,
                ),
                ..Default::default()
            },
        })
}

pub fn superimpose<T, const LHS_SIZE: usize, const RHS_SIZE: usize>(
    mut lhs: &mut [T; LHS_SIZE],
    range: Range<usize>,
    rhs: &[T; RHS_SIZE],
) where
    T: Copy,
{
    let mut ii = 0;
    for i in range.start..range.end {
        lhs[i] = rhs[ii];
        ii += 1;
    }
}

pub fn from_u16(v: u16) -> [u8; 2] {
    let mut ret: [u8; 2] = [0, 0];
    ret[0] = (v & 0x00ff) as u8;
    ret[1] = ((v & 0xff00) >> 8) as u8;
    ret
}
