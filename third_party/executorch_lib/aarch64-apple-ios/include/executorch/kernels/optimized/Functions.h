/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the root directory of this source tree.
 */

// clang-format off
#pragma once

#include <tuple>

#include <executorch/runtime/core/exec_aten/exec_aten.h> // at::Tensor etc.
#include <executorch/codegen/macros.h> // TORCH_API
#include <executorch/runtime/kernel/kernel_runtime_context.h>

// @generated by torchgen/gen_executorch.py from Functions.h

#include "NativeFunctions.h"

namespace torch {
namespace executor {


namespace aten {

// aten::add.out(Tensor self, Tensor other, *, Scalar alpha=1, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & add_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Tensor & other, const torch::executor::Scalar & alpha, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_add_out(context, self, other, alpha, out);
}


// aten::add.Scalar_out(Tensor self, Scalar other, Scalar alpha=1, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & add_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Scalar & other, const torch::executor::Scalar & alpha, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_add_scalar_out(context, self, other, alpha, out);
}


// aten::bmm.out(Tensor self, Tensor mat2, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & bmm_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Tensor & mat2, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_bmm_out(context, self, mat2, out);
}


// aten::div.out(Tensor self, Tensor other, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & div_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Tensor & other, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_div_out(context, self, other, out);
}


// aten::div.Scalar_out(Tensor self, Scalar other, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & div_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Scalar & other, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_div_scalar_out(context, self, other, out);
}


// aten::exp.out(Tensor self, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & exp_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_exp_out(context, self, out);
}


// aten::le.Scalar_out(Tensor self, Scalar other, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & le_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Scalar & other, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_le_scalar_out(context, self, other, out);
}


// aten::le.Tensor_out(Tensor self, Tensor other, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & le_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Tensor & other, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_le_tensor_out(context, self, other, out);
}


// aten::linear.out(Tensor input, Tensor weight, Tensor? bias=None, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & linear_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & input, const torch::executor::Tensor & weight, const torch::executor::optional<torch::executor::Tensor> & bias, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_linear_out(context, input, weight, bias, out);
}


// aten::mul.out(Tensor self, Tensor other, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & mul_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Tensor & other, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_mul_out(context, self, other, out);
}


// aten::mul.Scalar_out(Tensor self, Scalar other, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & mul_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Scalar & other, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_mul_scalar_out(context, self, other, out);
}


// aten::native_layer_norm.out(Tensor input, SymInt[] normalized_shape, Tensor? weight, Tensor? bias, float eps, *, Tensor(a!) out0, Tensor(b!) out1, Tensor(c!) out2) -> (Tensor(a!), Tensor(b!), Tensor(c!))
TORCH_API inline ::std::tuple<torch::executor::Tensor &,torch::executor::Tensor &,torch::executor::Tensor &> native_layer_norm_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & input, torch::executor::ArrayRef<int64_t> normalized_shape, const torch::executor::optional<torch::executor::Tensor> & weight, const torch::executor::optional<torch::executor::Tensor> & bias, double eps, torch::executor::Tensor & out0, torch::executor::Tensor & out1, torch::executor::Tensor & out2) {
    return ::torch::executor::native::opt_native_layer_norm_out(context, input, normalized_shape, weight, bias, eps, out0, out1, out2);
}


// aten::neg.out(Tensor self, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & neg_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_neg_out(context, self, out);
}


// aten::sub.out(Tensor self, Tensor other, *, Scalar alpha=1, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & sub_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Tensor & other, const torch::executor::Scalar & alpha, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_sub_out(context, self, other, alpha, out);
}


// aten::sub.Scalar_out(Tensor self, Scalar other, Scalar alpha=1, *, Tensor(a!) out) -> Tensor(a!)
TORCH_API inline torch::executor::Tensor & sub_outf(torch::executor::KernelRuntimeContext & context, const torch::executor::Tensor & self, const torch::executor::Scalar & other, const torch::executor::Scalar & alpha, torch::executor::Tensor & out) {
    return ::torch::executor::native::opt_sub_scalar_out(context, self, other, alpha, out);
}

} // namespace aten

} // namespace executor
} // namespace torch