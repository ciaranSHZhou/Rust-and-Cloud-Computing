#pragma once

// @generated by torchgen/gen.py from NativeFunction.h

#include <c10/core/Scalar.h>
#include <c10/core/Storage.h>
#include <c10/core/TensorOptions.h>
#include <c10/util/Deprecated.h>
#include <c10/util/Optional.h>
#include <c10/core/QScheme.h>
#include <ATen/core/Reduction.h>
#include <ATen/core/Tensor.h>
#include <tuple>
#include <vector>


namespace at {
namespace native {
TORCH_API ::std::tuple<at::Tensor &,at::Tensor &> _native_multi_head_attention_out(const at::Tensor & query, const at::Tensor & key, const at::Tensor & value, int64_t embed_dim, int64_t num_head, const at::Tensor & qkv_weight, const at::Tensor & qkv_bias, const at::Tensor & proj_weight, const at::Tensor & proj_bias, const c10::optional<at::Tensor> & mask, bool need_weights, bool average_attn_weights, c10::optional<int64_t> mask_type, at::Tensor & out0, at::Tensor & out1);
TORCH_API ::std::tuple<at::Tensor,at::Tensor> native_multi_head_attention(const at::Tensor & query, const at::Tensor & key, const at::Tensor & value, int64_t embed_dim, int64_t num_head, const at::Tensor & qkv_weight, const at::Tensor & qkv_bias, const at::Tensor & proj_weight, const at::Tensor & proj_bias, const c10::optional<at::Tensor> & mask={}, bool need_weights=true, bool average_attn_weights=true, c10::optional<int64_t> mask_type=c10::nullopt);
} // namespace native
} // namespace at