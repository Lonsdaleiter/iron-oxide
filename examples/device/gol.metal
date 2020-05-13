#include <metal_stdlib>

using namespace metal;

struct ToQuadVertex {
    float2 position;
	packed_float4 colour;
};

struct ToQuadFragment {
    float4 position [[position]];
    float4 colour;
};

vertex ToQuadFragment quad_v(device ToQuadVertex* vertexArray [[ buffer(0) ]],
                         unsigned int vid [[ vertex_id ]])
{
    ToQuadFragment out;
    out.position = float4(vertexArray[vid].position, 0.0, 1.0);
    out.colour = vertexArray[vid].colour;
    return out;
}

fragment float4 quad_f(ToQuadFragment in [[stage_in]])
{
    return in.colour;
}

// undefined behavior if all pixels aren't either fully solid or transparent
kernel void update_game(texture2d<float, access::read> inTexture [[texture(0)]],
                        texture2d<float, access::write> outTexture [[texture(1)]],
                        uint2 gid [[thread_position_in_grid]])
{
    float4 colourAtTexel = inTexture.read(gid);

    float n1 = inTexture.read(uint2(gid.x, gid.y + 1)).a;
    float n2 = inTexture.read(uint2(gid.x + 1, gid.y + 1)).a;
    float n3 = inTexture.read(uint2(gid.x - 1, gid.y + 1)).a;
    float n4 = inTexture.read(uint2(gid.x, gid.y - 1)).a;
    float n5 = inTexture.read(uint2(gid.x + 1, gid.y - 1)).a;
    float n6 = inTexture.read(uint2(gid.x - 1, gid.y - 1)).a;
    float n7 = inTexture.read(uint2(gid.x + 1, gid.y)).a;
    float n8 = inTexture.read(uint2(gid.x - 1, gid.y)).a;

    float sum = n1 + n2 + n3 + n4 + n5 + n6 + n7 + n8;

    // fewer than 2 neighbours = death, greater than 3 neighbours = death
    // dead cell with 3 neighbours comes alive
    if (colourAtTexel.a == 1.0 && (sum > 3.0 || sum < 2.0)) {
        outTexture.write(float4(0.0, 0.0, 0.0, 0.0), gid);
    } else if (colourAtTexel.a == 0.0 && sum == 3.0) {
        outTexture.write(float4(1.0, 1.0, 1.0, 1.0), gid);
    } else {
        outTexture.write(colourAtTexel, gid);
    }
}
