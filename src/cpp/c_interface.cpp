#include "c_interface.h"

#include "executorch/extension/module/module.h"
#include "executorch/extension/tensor/tensor.h"
#include "executorch/runtime/core/data_loader.h"
#include "executorch/runtime/executor/program.h"

#include <iostream>
#include <memory>
#include <vector>

using executorch::extension::Module;
using executorch::runtime::Error;
using executorch::runtime::EValue;
using torch::executor::ScalarType;
using torch::executor::Tensor;
using torch::executor::TensorImpl;

#ifdef __cplusplus
extern "C" {
#endif

struct CModule {
  Module *internal;
} typedef CModule;

CModule *c_new_module(const char *file_path) {
  return new CModule{new Module(file_path)};
}

void c_drop_module(CModule *module) {
  delete module->internal;
  delete module;
}

int c_load(CModule *module) {
  auto error = module->internal->load();
  if (error != Error::Ok) {
    return static_cast<int>(error);
  }
  return 0;
}

int c_forward(CModule *module, float *input, int32_t input_dim,
              int32_t input_sizes[], int32_t output_dim, int32_t output_sizes[],
              int32_t *found_output_dim, int32_t *found_output_sizes,
              float *output) {
  TensorImpl tensor_impl(ScalarType::Float, input_dim, input_sizes, input);
  auto result = module->internal->forward({EValue(Tensor(&tensor_impl))});
  if (!result.ok()) {
    return static_cast<int>(result.error());
  }

  const auto output_tensor = result->at(0).toTensor();
  // Copy the output dim
  if (output_tensor.dim() < 0) {
    return -1;
  }

  *found_output_dim = output_tensor.dim();
  if (output_tensor.dim() != output_dim) {
    return -2;
  }

  // Copy the output sizes
  auto output_tensor_size = output_tensor.sizes();
  for (int i = 0; i < output_dim; i++) {
    found_output_sizes[i] = output_tensor_size[i];
  }
  for (int i = output_dim; i < output_tensor.dim(); i++) {
    if (output_tensor_size[i] != output_sizes[i]) {
      return -3;
    }
  }

  // Copy the output tensor
  const auto output_data = output_tensor.const_data_ptr<float>();
  auto output_size = output_tensor.numel();
  std::copy(output_data, output_data + output_size, output);
  return 0;
}

// int c_forward(CModule *module, float *input, int dim, int32_t sizes[],
//               float **output) {
//   TensorImpl tensor_impl(ScalarType::Float, dim, sizes, input);
//   auto result = module->internal->forward({EValue(Tensor(&tensor_impl))});
//   if (!result.ok()) {
//     return static_cast<int>(result.error());
//   }
//   *output = std::move(result->at(0)).toTensor().data_ptr<float>();
//   return 0;
// }

#ifdef __cplusplus
}
#endif
