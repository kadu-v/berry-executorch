#include "c_interface.h"

#include "executorch/runtime/executor/program.h"
#include <executorch/extension/module/module.h>
#include <executorch/extension/tensor/tensor.h>
#include <executorch/runtime/core/data_loader.h>

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

int c_forward(CModule *module, float *input, int dim, int32_t sizes[],
              float **output) {
  TensorImpl tensor_impl(ScalarType::Float, dim, sizes, input);
  auto result = module->internal->forward({EValue(Tensor(&tensor_impl))});
  if (!result.ok()) {
    return static_cast<int>(result.error());
  }
  *output = std::move(result->at(0)).toTensor().data_ptr<float>();
  return 0;
}

#ifdef __cplusplus
}
#endif
