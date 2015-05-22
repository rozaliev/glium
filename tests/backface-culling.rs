#[macro_use]
extern crate glium;

use glium::Surface;

mod support;

#[test]
fn cull_clockwise() {
    let display = support::build_display();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display, vec![
            Vertex { position: [-1.0,  1.0] },      // top-left
            Vertex { position: [ 1.0,  1.0] },      // top-right
            Vertex { position: [-1.0, -1.0] },      // bottom-left
            Vertex { position: [ 1.0, -1.0] }       // bottom-right
        ])
    };

    // first triangle covers the top-left side of the screen and is clockwise
    // second triangle covers the bottom-right side of the screen and is ccw
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TrianglesList(vec![0u16, 1, 2, 1, 2, 3]));

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 140

                out vec4 color;
                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        110 => {
            vertex: "
                #version 110

                attribute vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        100 => {
            vertex: "
                #version 100

                attribute lowp vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
    ).unwrap();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
        &glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockWise,
            .. Default::default()
        }).unwrap();

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    assert_eq!(read_back[0].last().unwrap(), &(255, 0, 0, 255));
    assert_eq!(read_back.last().unwrap()[0], (0, 0, 0, 0));

    display.assert_no_error(None);
}

#[test]
fn cull_counterclockwise() {
    let display = support::build_display();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display, vec![
            Vertex { position: [-1.0,  1.0] },      // top-left
            Vertex { position: [ 1.0,  1.0] },      // top-right
            Vertex { position: [-1.0, -1.0] },      // bottom-left
            Vertex { position: [ 1.0, -1.0] }       // bottom-right
        ])
    };

    // first triangle covers the top-left side of the screen and is clockwise
    // second triangle covers the bottom-right side of the screen and is ccw
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TrianglesList(vec![0u16, 1, 2, 1, 2, 3]));

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 140

                out vec4 color;
                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        110 => {
            vertex: "
                #version 110

                attribute vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        100 => {
            vertex: "
                #version 100

                attribute lowp vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
    ).unwrap();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
        &glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
            .. Default::default()
        }).unwrap();

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    assert_eq!(read_back[0].last().unwrap(), &(0, 0, 0, 0));
    assert_eq!(read_back.last().unwrap()[0], (255, 0, 0, 255));

    display.assert_no_error(None);
}

#[test]
fn cull_clockwise_trianglestrip() {
    let display = support::build_display();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display, vec![
            Vertex { position: [-1.0,  1.0] },      // top-left
            Vertex { position: [ 1.0,  1.0] },      // top-right
            Vertex { position: [-1.0, -1.0] },      // bottom-left
            Vertex { position: [ 1.0, -1.0] }       // bottom-right
        ])
    };

    // both triangles are clockwise
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TriangleStrip(vec![0u16, 1, 2, 3]));

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 140

                out vec4 color;
                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        110 => {
            vertex: "
                #version 110

                attribute vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        100 => {
            vertex: "
                #version 100

                attribute lowp vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
    ).unwrap();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
        &glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockWise,
            .. Default::default()
        }).unwrap();

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    assert_eq!(read_back[0][0], (0, 0, 0, 0));
    assert_eq!(read_back.last().unwrap().last().unwrap(), &(0, 0, 0, 0));

    display.assert_no_error(None);
}

#[test]
fn cull_counterclockwise_trianglestrip() {
    let display = support::build_display();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display, vec![
            Vertex { position: [-1.0,  1.0] },      // top-left
            Vertex { position: [ 1.0,  1.0] },      // top-right
            Vertex { position: [-1.0, -1.0] },      // bottom-left
            Vertex { position: [ 1.0, -1.0] }       // bottom-right
        ])
    };

    // both triangles are clockwise
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TriangleStrip(vec![0u16, 1, 2, 3]));

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 140

                out vec4 color;
                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        110 => {
            vertex: "
                #version 110

                attribute vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
        100 => {
            vertex: "
                #version 100

                attribute lowp vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100

                void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "
        },
    ).unwrap();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
        &glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
            .. Default::default()
        }).unwrap();

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    assert_eq!(read_back[0][0], (255, 0, 0, 255));
    assert_eq!(read_back.last().unwrap().last().unwrap(), &(255, 0, 0, 255));

    display.assert_no_error(None);
}
