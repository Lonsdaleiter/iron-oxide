#include <metal_stdlib>

using namespace metal;

struct VertexStruct {
	float4 position;
	float4 colour;
};

struct ToFragStruct {
    float4 position [[position]];
    float4 colour;
};

// vertex shader function
vertex ToFragStruct vertex_shader(device VertexStruct* vertexArray [[ buffer(0) ]],
                                  unsigned int vid [[ vertex_id ]])
{
    ToFragStruct out;
    out.position = vertexArray[vid].position;
    out.colour = vertexArray[vid].colour;
    return out;
}

// fragment shader function
fragment float4 fragment_shader(ToFragStruct in [[stage_in]])
{
    return in.colour;
}
