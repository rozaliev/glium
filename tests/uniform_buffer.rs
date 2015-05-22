#[macro_use]
extern crate glium;
extern crate rand;

use glium::Surface;

mod support;

#[test]
fn uniform_buffer_creation() {
    let display = support::build_display();

    glium::uniforms::UniformBuffer::new_if_supported(&display, 12);

    display.assert_no_error(None);
}

#[test]
fn uniform_buffer_mapping_read() {
    let display = support::build_display();

    let mut vb = match glium::uniforms::UniformBuffer::new_if_supported(&display, 12) {
        None => return,
        Some(b) => b
    };

    let mapping = vb.map();
    assert_eq!(*mapping, 12);

    display.assert_no_error(None);
}

#[test]
fn uniform_buffer_mapping_write() {
    let display = support::build_display();

    let mut vb = match glium::uniforms::UniformBuffer::new_if_supported(&display, 6) {
        None => return,
        Some(b) => b
    };

    {
        let mut mapping = vb.map();
        *mapping = 15;
    }

    let mapping = vb.map();
    assert_eq!(*mapping, 15);

    display.assert_no_error(None);
}

#[test]
fn uniform_buffer_read() {
    let display = support::build_display();

    let vb = match glium::uniforms::UniformBuffer::new_if_supported(&display, 12) {
        None => return,
        Some(b) => b
    };

    let data = match vb.read_if_supported() {
        Some(d) => d,
        None => return
    };

    assert_eq!(data, 12);

    display.assert_no_error(None);
}

#[test]
fn uniform_buffer_write() {
    let display = support::build_display();

    let mut vb = match glium::uniforms::UniformBuffer::new_if_supported(&display, 5) {
        None => return,
        Some(b) => b
    };

    vb.upload(24);

    let data = match vb.read_if_supported() {
        Some(d) => d,
        None => return
    };

    assert_eq!(data, 24);

    display.assert_no_error(None);
}

#[test]
fn block() {
    let display = support::build_display();

    let (vb, ib) = support::build_rectangle_vb_ib(&display);

    let program = glium::Program::from_source(&display,
        "
            #version 110

            attribute vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
        "
            #version 330
            uniform layout(std140);

            uniform MyBlock {
                vec3 color;
            };

            void main() {
                gl_FragColor = vec4(color, 1.0);
            }
        ",
        None);

    // ignoring test in case of compilation error (version may not be supported)
    let program = match program {
        Ok(p) => p,
        Err(_) => return
    };

    let buffer = match glium::uniforms::UniformBuffer::new_if_supported(&display, (1.0f32, 1.0f32, 0.0f32)) {
        None => return,
        Some(b) => b
    };

    let uniforms = uniform!{
        MyBlock: &buffer
    };

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vb, &ib, &program, &uniforms, &Default::default()).unwrap();

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 255, 0, 255));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn block_wrong_type() {
    let display = support::build_display();

    let (vb, ib) = support::build_rectangle_vb_ib(&display);

    let program = glium::Program::from_source(&display,
        "
            #version 110

            attribute vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
        "
            #version 330
            uniform layout(std140);

            uniform MyBlock {
                vec3 color;
            };

            void main() {
                gl_FragColor = vec4(color, 1.0);
            }
        ",
        None);

    // ignoring test in case of compilation error (version may not be supported)
    let program = match program {
        Ok(p) => p,
        Err(_) => return
    };

    let buffer = match glium::uniforms::UniformBuffer::new_if_supported(&display, 2) {
        None => return,
        Some(b) => b
    };

    let uniforms = uniform!{
        MyBlock: &buffer
    };

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);

    match target.draw(&vb, &ib, &program, &uniforms, &Default::default()) {
        Err(glium::DrawError::UniformBlockLayoutMismatch { ref name })
            if name == &"MyBlock" => (),
        a => panic!("{:?}", a)
    }

    display.assert_no_error(None);
}

#[test]
fn buffer_write() {
    let display = support::build_display();

    let mut buf = match glium::uniforms::UniformBuffer::new_if_supported(&display, (5, 3)) {
        None => return,
        Some(b) => b
    };

    {
        let mut mapping = buf.map();
        mapping.1 = 8;
    }

    let mapping = buf.map();
    assert_eq!(*mapping, (5, 8));

    display.assert_no_error(None);
}

#[test]
fn persistent_block_race_condition() {
    let display = support::build_display();

    let (vb, ib) = support::build_rectangle_vb_ib(&display);

    let program = glium::Program::from_source(&display,
        "
            #version 110

            attribute vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
        "
            #version 330
            uniform layout(std140);

            uniform MyBlock {
                vec3 color;
            };

            void main() {
                gl_FragColor = vec4(color, 1.0);
            }
        ",
        None);

    // ignoring test in case of compilation error (version may not be supported)
    let program = match program {
        Ok(p) => p,
        Err(_) => return
    };

    let mut buffer = match glium::uniforms::UniformBuffer::new_if_supported(&display, (0.5f32, 0.5f32, 0.5f32)) {
        None => return,
        Some(b) => b
    };

    // checking for synchronization issues by quickly drawing and modifying the buffer
    let texture = support::build_renderable_texture(&display);
    let mut target = texture.as_surface();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    for _ in (0 .. 1000) {
        {
            let mut mapping = buffer.map();
            (*mapping).0 = rand::random();
            (*mapping).1 = rand::random();
            (*mapping).2 = rand::random();
        }

        target.draw(&vb, &ib, &program, &uniform!{
            MyBlock: &buffer
        }, &Default::default()).unwrap();
    }
    {
        let mut mapping = buffer.map();
        (*mapping).0 = 1.0;
        (*mapping).1 = 1.0;
        (*mapping).2 = 1.0;
    }
    target.draw(&vb, &ib, &program, &uniform!{
        MyBlock: &buffer
    }, &Default::default()).unwrap();
    {
        let mut mapping = buffer.map();
        (*mapping).0 = 0.0;
        (*mapping).1 = 0.0;
        (*mapping).2 = 0.0;
    }

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 255, 255, 255));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn empty_uniform_buffer() {
    let display = support::build_display();

    let _ = match glium::uniforms::UniformBuffer::new_if_supported(&display, ()) {
        None => return,
        Some(b) => b
    };

    display.assert_no_error(None);
}
