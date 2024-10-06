#pragma once
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CModule CModule;

CModule *c_new_module(const char *file_path);

void c_drop_module(CModule *module);

int c_load(CModule *module);

// This function executes the forward method of the module.
// Safety:
// - input: Not owned by C++, and must be valid for the lifetime of the call.
// - dim: The number of dimensions in the input tensor.
// - sizes: The size of the input tensor at each dimension.
// - output: Moved to the caller.
int c_forward(CModule *module, float *input, int dim, int32_t sizes[],
              float **output);

#ifdef __cplusplus
}
#endif