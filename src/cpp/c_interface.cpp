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
extern "C"
{
#endif
  struct CModule
  {
    Module *internal;
  } typedef CModule;

  CModule *c_new_module(const char *file_path)
  {
    /** IMPORTANT: If the Default LoadMode (Mlock) is enable, the following error will be thrown:
     *  Error: Mlock failed: Cannot allocate memory Out of memory
     */
    return new CModule{new Module(file_path, Module::LoadMode::Mmap)};
  }

  void c_drop_module(CModule *module)
  {
    delete module->internal;
    delete module;
  }

  int c_load(CModule *module)
  {
    auto error = module->internal->load();
    if (error != Error::Ok)
    {
      return static_cast<int>(error);
    }
    return 0;
  }

  CTensor c_forward(CModule *module, float *input, int32_t input_dim, int32_t *input_sizes)
  {
    TensorImpl tensor_impl(ScalarType::Float, input_dim, input_sizes, input);

    auto result = module->internal->forward({EValue(Tensor(&tensor_impl))});
    if (!result.ok())
    {
      return CTensor{
          static_cast<int>(result.error()),
          nullptr,
          -1,
          nullptr};
    }

    const auto output_tensor = result->at(0).toTensor();
    const float *output_ptr = output_tensor.const_data_ptr<float>();

    float *output = new float[output_tensor.numel()];
    std::copy(output_ptr, output_ptr + output_tensor.numel(), output);

    int32_t output_dim = output_tensor.dim();

    int32_t *output_sizes = new int32_t[output_tensor.dim()];
    std::copy(output_tensor.sizes().begin(), output_tensor.sizes().end(), output_sizes);
    // Copy the output tensor to the output array and this is not owned by the callee

    return CTensor{
        0,
        output,
        output_dim,
        output_sizes};
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
