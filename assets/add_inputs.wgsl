@group(0) @binding(0)
var texture_a: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1)
var texture_b: texture_storage_2d<rgba8unorm, read_write>;

@group(1) @binding(0)
var<uniform> current_texture: u32;

struct Action {
    pos: vec2<f32>,
    typ: u32
}

@group(2) @binding(0)
var<storage, read> input_actions: array<Action>;


fn action_type_to_colour(action: Action) -> vec4<f32> {
    if action.typ > 1u { // non-existant action type
        return vec4<f32>(1., 0., 0., 1.);
    }

    return select(vec4<f32>(1.), vec4<f32>(0., 0., 0., 1.), action.typ == 0u);
}

fn apply_action_at_location(action: Action, location: vec2<i32>) {
    let colour = action_type_to_colour(action);

    storageBarrier();
    // if current_texture == 1u {
        textureStore(texture_a, location, colour);
    // } else {
        textureStore(texture_b, location, colour);
    // }
}


@compute @workgroup_size(8, 8, 1)
fn add_inputs(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location_to_write_at = vec2<i32>(invocation_id.xy);
    let location_to_compare = vec2<f32>(invocation_id.xy);

    let length = arrayLength(&input_actions);

    // loop backwards so that the latest action takes priority (overwriting earlier actions)
    // for (var i = length; i >= 0u; i--) {
        let action = input_actions[0];

        if all(action.pos == location_to_compare) { // exact
        // if all(location_to_compare <= action.pos) { // cover area below
        // if all(location_to_compare <= vec2<f32>(length)) { // cover action type
            apply_action_at_location(action, location_to_write_at);

            // break
        }
        // else {
        //     apply_action_at_location(Action(1u, vec2<f32>(0.)), location_to_write_at);
        // }
    // }
}
