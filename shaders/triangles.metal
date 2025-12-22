
// Edge function (signed area * 2)
float edge(const float2 v0, const float2 v1, const float2 p, const float dx, const float dy) {
    return (p.x - v0.x) * (dy) - (p.y - v0.y) * (dx);
}

struct Vertex {
    const float4 position;
    const float4 uv;
};

kernel void ComputeShader (
    device const uint&   pitch             [[ buffer(0 ) ]],  // from sdl2 for padding
    device const uint&   width             [[ buffer(1 ) ]],  //
    device const uint&   height            [[ buffer(2 ) ]],  //

    device const float4& camera_position   [[ buffer(3 ) ]],  // camera position
    device const float4& camera_rotation   [[ buffer(4 ) ]],  // camera rotation

    device const Vertex* const vertex_buffer     [[ buffer(5 ) ]],  // triangle data

    device const float4* const normals           [[ buffer(6 ) ]],  // number
    device const uint4* const  triangles_buffer  [[ buffer(7 ) ]],  // triangle indices
    device const uint* const   num_triangles     [[ buffer(8 ) ]],  // number of triangle indices (changed.... no longer that but both tri id's and counts for the binning)

    device const uchar4* const texture_buffer    [[ buffer(9 ) ]],  // texture data

    device float*  depth_buffer  [[ buffer(10) ]],
    device uchar*  pixels        [[ buffer(11) ]],

    uint2 gid [[ thread_position_in_grid ]]
) {
    constexpr uint MAX_TRIANGLES_PER_BIN = 64;
    constexpr uint CELL_SIZE = 4;  // seems to be a solid point for this

    if (gid.x >= metal::ceil(float(width) / float(CELL_SIZE)) * CELL_SIZE || gid.y >= metal::ceil(float(height) / float(CELL_SIZE)) * CELL_SIZE) {
        return;
    }

    const uint bin_width = metal::ceil(float(width) / float(CELL_SIZE));  // used for the binned triangles array

    const uint2 gid_base = gid * CELL_SIZE;
    const uint2 gid_max = gid_base + CELL_SIZE;

    const float3 sun_direction = metal::normalize(float3(0.5, 1.0, -0.7));

    const uint base_index = (gid.x + gid.y * bin_width) * MAX_TRIANGLES_PER_BIN;
    const uint tris_in_bin = num_triangles[base_index];
    for (uint bin_id = 0; bin_id < tris_in_bin; bin_id++) {
        const uint tri_id = num_triangles[base_index + bin_id + 1];
        const uint4 triangle = triangles_buffer[tri_id];
        const float3 triangle_normal = normals[triangle.w & 0xFFFF].xyz;
        // I think that's working, but definitely verify bc/ I have no clue (orthographic projection makes this hard to see)
        //if (metal::dot(triangle_normal, camera_rotation.xyz) >= 0.0) { continue; }  // continue;     make this work at some point ig

        device const Vertex* const tri_1 = &vertex_buffer[triangle.x];
        device const Vertex* const tri_2 = &vertex_buffer[triangle.y];
        device const Vertex* const tri_3 = &vertex_buffer[triangle.z];

        const float3 v1 = tri_1->position.xyz;
        const float3 v2 = tri_2->position.xyz;
        const float3 v3 = tri_3->position.xyz;

        // transforming the vertexes based on rotation and position
        const uint maxX = uint(metal::ceil(metal::max(v1.x, metal::max(v2.x, v3.x))));
        const uint minX = uint(metal::floor(metal::min(v1.x, metal::min(v2.x, v3.x))));
        const uint maxY = uint(metal::ceil(metal::max(v1.y, metal::max(v2.y, v3.y))));
        const uint minY = uint(metal::floor(metal::min(v1.y, metal::min(v2.y, v3.y))));
        if (maxY < gid_base.y || minY > gid_max.y || maxX < gid_base.x || minX > gid_max.x) continue;

        const float light_intensity = metal::dot(triangle_normal, sun_direction) * 0.5 + 0.5;
        uchar texture_index = uchar(triangle.w >> 16);

        const float dx_32 = v3.x - v2.x;
        const float dy_32 = v3.y - v2.y;
        const float dx_13 = v1.x - v3.x;
        const float dy_13 = v1.y - v3.y;

        const float area = 1.0 / edge(v1.xy, v2.xy, v3.xy, (v2.x - v1.x), (v2.y - v1.y));
        for (uint x = gid_base.x; x < gid_max.x; x++) {
            for (uint y = gid_base.y; y < gid_max.y; y++) {
                const float2 float_coord = float2(float(x), float(y));

                const float w0 = edge(v2.xy, v3.xy, float_coord, dx_32, dy_32) * area;
                const float w1 = edge(v3.xy, v1.xy, float_coord, dx_13, dy_13) * area;
                const float w2 = 1.0 - w0 - w1;

                const float depth = v1.z * w0 + v2.z * w1 + v3.z * w2;
                const float uv_u = 16.0 * (w0 * tri_1->uv.x + w1 * tri_2->uv.x + w2 * tri_3->uv.x);
                const float uv_v = 16.0 * (w0 * tri_1->uv.y + w1 * tri_2->uv.y + w2 * tri_3->uv.y);

                const uint uv_x = uint(uv_u);
                const uint uv_y = uint(uv_v);
                uchar4 texture_col = texture_buffer[texture_index * 256 + uv_y * 16 + uv_x];

                const uint depth_index = x + y * width;
                if (texture_col.w == 255 || x < minX || x >= width || y < minY || y >= height || w0 < 0.0 || w1 < 0.0 || w2 < 0.0 || depth >= depth_buffer[depth_index]) continue;
                depth_buffer[depth_index] = depth;

                const uint pixel_index = x * 3 + (height - y) * pitch;
                pixels[pixel_index + 0] = uchar(texture_col.x * light_intensity);
                pixels[pixel_index + 1] = uchar(texture_col.y * light_intensity);
                pixels[pixel_index + 2] = uchar(texture_col.z * light_intensity);
            }
        }
    }
}

