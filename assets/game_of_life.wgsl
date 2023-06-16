@group(0) @binding(0)
var texture_a: texture_storage_2d<rgba8unorm, read_write>; //TODO change to f32?
@group(0) @binding(1)
var texture_b: texture_storage_2d<rgba8unorm, read_write>;

@group(1) @binding(0)
var<uniform> current_texture: u32;


fn is_alive(location: vec2<i32>, offset_x: i32, offset_y: i32) -> f32 {
    if current_texture == 1u {
        return textureLoad(texture_a, location + vec2<i32>(offset_x, offset_y)).x;
    } else {
        return textureLoad(texture_b, location + vec2<i32>(offset_x, offset_y)).x;
    }
}

fn count_alive(location: vec2<i32>) -> f32 {
    return is_alive(location, -1, -1, )
        + is_alive(location, -1,  0)
        + is_alive(location, -1,  1)
        + is_alive(location,  0, -1)
        + is_alive(location,  0,  1)
        + is_alive(location,  1, -1)
        + is_alive(location,  1,  0)
        + is_alive(location,  1,  1);
}


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let alive_count = floor(count_alive(location) + .5);

    var alive = alive_count == 3.; // dead or alive with 3 neighbours = alive
    alive = alive || (alive_count == 2. && bool(is_alive(location, 0, 0))); // 2 neighbours and is alive = stays alive

    let out = vec4<f32>(f32(alive));


    storageBarrier();
    if current_texture == 0u {
        textureStore(texture_a, location, out);
    } else {
        textureStore(texture_b, location, out);
    }
}
