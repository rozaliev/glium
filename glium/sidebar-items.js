initSidebarItems({"type":[["Display","The main object of this library. Controls the whole display."],["VertexFormat","Describes the layout of each vertex in a vertex buffer."]],"mod":[["backend","The `backend` module allows one to link between glium and the OpenGL context.."],["buffer","Buffers management in glium"],["debug",""],["draw_parameters","Describes miscellaneous parameters to be used when drawing."],["framebuffer","Framebuffers allow you to customize the color, depth and stencil buffers you will draw on."],["glutin",""],["index","In order to draw, you need to provide a source of indices which is used to link the vertices together into *primitives*."],["pixel_buffer","Pixel buffers are buffers that contain two-dimensional texture data."],["program",""],["render_buffer","DEPRECATED. Use `framebuffer::*` instead."],["texture","A texture is an image loaded in video memory, which can be sampled in your shaders."],["uniforms","A uniform is a global variable in your program. In order to draw something, you will need to give `glium` the values of all your uniforms. Objects that implement the `Uniform` trait are here to do that."],["vertex","Contains everything related to vertex sources."]],"struct":[["BlitTarget","Area of a surface in pixels. Similar to a `Rect` except that dimensions can be negative."],["Frame","Implementation of `Surface`, targeting the default framebuffer."],["IndexBuffer","A list of indices loaded in the graphics card's memory."],["LinearSyncFence","Prototype for a `SyncFence`."],["Program","A combination of shaders linked together."],["Rect","Area of a surface in pixels."],["SyncFence","Provides a way to wait for a server-side operation to be finished."],["Version","Describes a version."],["VertexBuffer","A list of vertices loaded in the graphics card's memory."]],"fn":[["get_supported_glsl_version","Given an API version, this function returns the GLSL version that the implementation is required to support."]],"macro":[["assert_no_gl_error!","Calls the `assert_no_error` method on a `glium::Display` instance with file and line number information."],["implement_vertex!","Implements the `glium::vertex::Vertex` trait for the given type."],["program!","Builds a program depending on the GLSL version supported by the backend."],["uniform!","Returns an implementation-defined type which implements the `Uniform` trait."]],"enum":[["Api","Describes an OpenGL-related API."],["DrawError","Error that can happen while drawing."],["GliumCreationError","Error that can happen while creating a glium display."],["Handle","Handle to a shader or a program."],["ProgramCreationError","Error that can be triggered when creating a `Program`."]],"trait":[["DisplayBuild","Objects that can build a facade object."],["GlObject","Trait for objects that are OpenGL objects."],["Surface","Object that can be drawn upon."]]});