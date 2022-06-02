#pragma once
#include <assert.h>
#include <memory>
#include <vector>
#include <Eigen/Core>
#include <Eigen/Geometry>
#include "rust/cxx.h"

using stride_t = Eigen::Stride<Eigen::Dynamic, Eigen::Dynamic>;
using stride_x1_t = Eigen::Stride<Eigen::Dynamic, 1>;

namespace Eigen {
using Matrix2x3d = Matrix<double, 2, 3>;
}

#define use_reloc(T)                                                    \
  namespace rxx {                                                       \
  using T = Eigen::T;                                                   \
  using Map_##T = Eigen::Map<T>;                                        \
  using Map_##T##_const = Eigen::Map<const T>;                          \
  }                                                                     \
  template<> struct rust::IsRelocatable<rxx::T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_const>: std::true_type {};

#define use_reloc_matx_stride(T)                                        \
  use_reloc(T)                                                          \
  namespace rxx {                                                       \
    using Map_##T##_stride=Eigen::Map<T, 0, stride_t>;                  \
    using Map_##T##_const_stride=Eigen::Map<const T, 0, stride_t>;      \
  }                                                                     \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_stride>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_const_stride>: std::true_type {};

#define use_reloc_vecx_stride(T)                                        \
  use_reloc(T)                                                          \
  namespace rxx {                                                       \
    using Map_##T##_stride=Eigen::Map<T, 0, stride_x1_t>;               \
    using Map_##T##_const_stride=Eigen::Map<const T, 0, stride_x1_t>;   \
  }                                                                     \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_stride>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_const_stride>: std::true_type {};


use_reloc(Vector2d)
use_reloc(Vector3d)
use_reloc(Vector4d)
use_reloc_vecx_stride(VectorXd)
use_reloc(Matrix2d)
use_reloc(Matrix3d)
use_reloc(Matrix4d)
use_reloc(Matrix2x3d)
use_reloc_matx_stride(MatrixXd)

use_reloc(Quaterniond)
use_reloc(AngleAxisd)

namespace rxx {

template<typename T1, typename T2, typename T3>
T3 op_add(const T1 &self_, const T2 &other) {return self_ + other;}
template<typename T1, typename T2, typename T3>
T3 op_sub(const T1 &self_, const T2 &other) {return self_ - other;}
template<typename T1, typename T2, typename T3>
T3 op_mul(const T1 &self_, const T2 &other) {return self_ * other;}

VectorXd VectorXd_new(size_t size) {return VectorXd(size);}
VectorXd VectorXd_clone(const VectorXd &v) {return VectorXd(v);}

Map_VectorXd Map_VectorXd_new(double *data, size_t size) {
  return Map_VectorXd(data, size);
}
Map_VectorXd_const Map_VectorXd_const_new(const double *data, size_t size) {
  return Map_VectorXd_const(data, size);
}

Map_VectorXd_stride Map_VectorXd_stride_new(double *data, size_t size, size_t s)
{
  return Map_VectorXd_stride(data, size, stride_x1_t(s, 1));
}
Map_VectorXd_const_stride Map_VectorXd_const_stride_new(const double *data, size_t size, size_t s)
{
  return Map_VectorXd_const_stride(data, size, stride_x1_t(s, 1));
}

MatrixXd MatrixXd_new(size_t rows, size_t cols) {return MatrixXd(rows, cols);}
MatrixXd MatrixXd_clone(const MatrixXd &v) {return MatrixXd(v);}

Map_MatrixXd Map_MatrixXd_new(double *data, size_t rows, size_t cols) {
  return Map_MatrixXd(data, rows, cols);
}
Map_MatrixXd_const Map_MatrixXd_const_new(const double *data, size_t rows, size_t cols) {
  return Map_MatrixXd_const(data, rows, cols);
}

Map_MatrixXd_stride Map_MatrixXd_stride_new(double *data, size_t rows, size_t cols, size_t s0, size_t s1)
{
  return Map_MatrixXd_stride(data, rows, cols, stride_t(s0, s1));
}
Map_MatrixXd_const_stride Map_MatrixXd_const_stride_new(const double *data, size_t rows, size_t cols, size_t s0, size_t s1)
{
  return Map_MatrixXd_const_stride(data, rows, cols, stride_t(s0, s1));
}

template<typename T>
Eigen::Map<T> Map_fixed_new(double *data) {return Eigen::Map<T>(data);}
template<typename T>
Eigen::Map<const T> Map_fixed_const_new(const double *data) {return Eigen::Map<const T>(data);}


} // namespace
