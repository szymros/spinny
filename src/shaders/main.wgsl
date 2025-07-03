struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_cords: vec2<f32>,
  @location(2) normal: vec3<f32>,
  @location(3) instance_transform_0: vec4<f32>,
  @location(4) instance_transform_1: vec4<f32>,
  @location(5) instance_transform_2: vec4<f32>,
  @location(6) instance_transform_3: vec4<f32>,
}


struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_cords: vec2<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) world_position: vec4<f32>

}

struct Camera {
  view_matrix: mat4x4<f32>,
  pos: vec4<f32>
}

@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Light {
  pos: vec3<f32>,
  color: vec3<f32>,
  view_proj: mat4x4<f32>
}

@group(0)@binding(1)
var<uniform> light:Light;


@vertex
fn vs_main(v_in: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let instance_matrix = mat4x4<f32>(
        v_in.instance_transform_0,
        v_in.instance_transform_1,
        v_in.instance_transform_2,
        v_in.instance_transform_3,
    );
    output.tex_cords = v_in.tex_cords;
    output.world_position = instance_matrix * vec4<f32>(v_in.position, 1.0);
    output.clip_position = camera.view_matrix * instance_matrix * vec4<f32>(v_in.position, 1.0);
    output.normal = normalize(instance_matrix * vec4<f32>(v_in.normal, 0.0)).xyz;
    return output;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

struct Material {
  ambient_str: vec3<f32>,
  diffuse_str: vec3<f32>,
  specular_str: vec3<f32>,
  shininess: f32
}

@group(2)@binding(0)
var<uniform> material:Material;

@group(3) @binding(0)
var shadow_texture: texture_depth_cube;
@group(3) @binding(1)
var shadow_texture_sampler: sampler_comparison;



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let light_dir = normalize(light.pos - in.world_position.xyz);
    let view_dir = normalize(camera.pos - in.world_position).xyz;
    let half_dir = normalize(view_dir + light_dir);
    let reflected = reflect(in.normal, half_dir);

    let ambient = 0.1 * light.color * material.ambient_str;
    let diffuse = max(dot(in.normal, light_dir), 0.0) * material.diffuse_str;
    let specular = pow(max(dot(in.normal, half_dir), 0.0), material.shininess) * material.specular_str;

    let l = in.world_position.xyz - light.pos;
    let biased_depth = length(l) / 100.0 - 0.005;
    let shadow = textureSampleCompare(shadow_texture, shadow_texture_sampler, l, biased_depth);

    let color = (ambient + shadow * (diffuse + specular)) * light.color;

    return vec4<f32>(color, 1.0) * textureSample(texture, texture_sampler, in.tex_cords);
    //return vec4<f32>(shadow,shadow,shadow, 1.0);
}
