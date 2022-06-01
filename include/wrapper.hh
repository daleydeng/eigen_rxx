#pragma once
#include <assert.h>
#include <memory>
#include <vector>
#include <Eigen/Core>
#include "rust/cxx.h"

#define use_reloc(T)                                                    \
  namespace eigen_rxx {                                                 \
  using stride_t = Eigen::Stride<Eigen::Dynamic, Eigen::Dynamic>;       \
  using T = Eigen::T;                                                   \
  using Map_##T = Eigen::Map<T>;                                        \
  using Map_##T##_const = Eigen::Map<const T>;                          \
  using Map_##T##_stride=Eigen::Map<T, 0, stride_t>;                    \
  using Map_##T##_const_stride=Eigen::Map<const T, 0, stride_t>;        \
  }                                                                     \
  template<> struct rust::IsRelocatable<eigen_rxx::T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<eigen_rxx::Map_##T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<eigen_rxx::Map_##T##_const>: std::true_type {}; \
  template<> struct rust::IsRelocatable<eigen_rxx::Map_##T##_stride>: std::true_type {}; \
  template<> struct rust::IsRelocatable<eigen_rxx::Map_##T##_const_stride>: std::true_type {};

use_reloc(Vector2d)
use_reloc(Vector3d)
use_reloc(Vector4d)
use_reloc(Matrix2d)
use_reloc(Matrix3d)
use_reloc(Matrix4d)
use_reloc(MatrixXd)

namespace eigen_rxx {

template<typename T>
T op_add(const T &self_, const T &other) {return self_ + other;}
template<typename T>
T op_sub(const T &self_, const T &other) {return self_ - other;}
template<typename T>
T op_mul(const T &self_, const T &other) {return self_ * other;}

MatrixXd MatrixXd_new(size_t rows, size_t cols) {return MatrixXd(rows, cols);}
MatrixXd MatrixXd_clone(const MatrixXd &v) {return MatrixXd(v);}

Map_MatrixXd Map_MatrixXd_new(double *data, size_t rows, size_t cols) {
  return Map_MatrixXd(data, rows, cols);
}
Map_MatrixXd_const Map_MatrixXd_const_new(const double *data, size_t rows, size_t cols) {
  return Map_MatrixXd_const(data, rows, cols);
}

Map_MatrixXd_stride Map_MatrixXd_stride_new(double *data, size_t rows, size_t cols, size_t sx, size_t sy)
{
  return Map_MatrixXd_stride(data, rows, cols, stride_t(sx, sy));
}
Map_MatrixXd_const_stride Map_MatrixXd_const_stride_new(const double *data, size_t rows, size_t cols, size_t sx, size_t sy)
{
  return Map_MatrixXd_const_stride(data, rows, cols, stride_t(sx, sy));
}

}
