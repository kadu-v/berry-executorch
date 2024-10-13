#pragma once
#include <cstdint>

#ifdef __cplusplus
extern "C"
{
#endif
    typedef struct CModule CModule;

    struct CTensor
    {
        int32_t error;
        const float *data;
        int32_t dim;
        int32_t *sizes;
    } typedef CTensor;

    CModule *c_new_module(const char *file_path);

    void c_drop_module(CModule *module);

    int c_load(CModule *module);

    CTensor c_forward(CModule *module, float *input, int32_t input_dim, int32_t *input_sizes);

    // // This function executes the forward method of the module.
    // // Safety:
    // // - input: Not owned by C++, and must be valid for the lifetime of the call.
    // // - dim: The number of dimensions in the input tensor.
    // // - sizes: The size of the input tensor at each dimension.
    // // - output: Moved to the caller.
    // int c_forward(CModule *module, float *input, int dim, int32_t sizes[],
    //               float **output);

#ifdef __cplusplus
}
#endif