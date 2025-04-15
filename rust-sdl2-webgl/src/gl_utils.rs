pub mod gl_types;   // use our own gl types
pub use glow::HasContext;

pub fn load_shader_program(gl : &glow::Context, vertex_shader_src: &str, fragment_shader_src: &str) -> Result<gl_types::ShaderProgram, String>
{
    if vertex_shader_src.is_empty() || fragment_shader_src.is_empty()
    {
        return Err("Empty vertex or fragment shader".to_string());
    }

    let program: gl_types::ShaderProgram;
    unsafe {
        // create the shader program
        program = match gl.create_program() {
            Err(error_txt) => return Err(error_txt),
            Ok(prog) => prog
        };
        
        // create the shaders
        let vertex_shader = match gl.create_shader(glow::VERTEX_SHADER) {
            Err(str) => return Err(str),
            Ok(shader) => shader
        };
        gl.shader_source(vertex_shader, vertex_shader_src);
        gl.compile_shader(vertex_shader);
        if !gl.get_shader_compile_status(vertex_shader)
        {
            return Err(gl.get_shader_info_log(vertex_shader));
        }
        
        let fragment_shader = match gl.create_shader(glow::FRAGMENT_SHADER) {
            Err(str) => return Err(str),
            Ok(shader) => shader
        };
        gl.shader_source(fragment_shader, fragment_shader_src);
        gl.compile_shader(fragment_shader);
        if !gl.get_shader_compile_status(fragment_shader)
        {
            return Err(gl.get_shader_info_log(fragment_shader));
        }

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        if !gl.get_program_link_status(program)
        {
            return Err(gl.get_program_info_log(program));
        }

        // we dont need the shader objects any more, only the program
        gl.detach_shader(program, vertex_shader);
        gl.delete_shader(vertex_shader);

        gl.detach_shader(program, fragment_shader);
        gl.delete_shader(fragment_shader);
    }
    return Ok(program);
}

pub fn unload_shader_program(gl : &glow::Context, program: &gl_types::ShaderProgram)
{
    unsafe {
        gl.delete_program(*program);
    }
}