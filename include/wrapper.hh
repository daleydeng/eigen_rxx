#pragma once
#include <assert.h>
#include <memory>
#include <vector>
#include <Eigen/Core>
#include <Eigen/Geometry>
#include "rust/cxx.h"

using stride_t = Eigen::Stride<Eigen::Dynamic, Eigen::Dynamic>;

#define use_reloc(T)                                                \
  namespace rxx {                                                 \
  using T = Eigen::T;                                                   \
  using Map_##T = Eigen::Map<T>;                                        \
  using Map_##T##_const = Eigen::Map<const T>;                          \
  }                                                                     \
  template<> struct rust::IsRelocatable<rxx::T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_const>: std::true_type {};

#define use_reloc_mat(T)                                                \
  namespace rxx {                                                 \
  using T = Eigen::T;                                                   \
  using Map_##T = Eigen::Map<T>;                                        \
  using Map_##T##_const = Eigen::Map<const T>;                          \
  using Map_##T##_stride=Eigen::Map<T, 0, stride_t>;                    \
  using Map_##T##_const_stride=Eigen::Map<const T, 0, stride_t>;        \
  }                                                                     \
  template<> struct rust::IsRelocatable<rxx::T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_const>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_stride>: std::true_type {}; \
  template<> struct rust::IsRelocatable<rxx::Map_##T##_const_stride>: std::true_type {};


use_reloc_mat(Vector2d)
use_reloc_mat(Vector3d)
use_reloc_mat(Vector4d)
use_reloc_mat(Matrix2d)
use_reloc_mat(Matrix3d)
use_reloc_mat(Matrix4d)
use_reloc_mat(MatrixXd)
use_reloc(Quaterniond)
use_reloc(AngleAxisd)

namespace rxx {

template<typename T1, typename T2, typename T3>
T3 op_add(const T1 &self_, const T2 &other) {return self_ + other;}
template<typename T1, typename T2, typename T3>
T3 op_sub(const T1 &self_, const T2 &other) {return self_ - other;}
template<typename T1, typename T2, typename T3>
T3 op_mul(const T1 &self_, const T2 &other) {return self_ * other;}

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

template<typename T>
Eigen::Map<T> Map_fixed_new(double *data) {return Eigen::Map<T>(data);}
template<typename T>
Eigen::Map<const T> Map_fixed_const_new(const double *data) {return Eigen::Map<const T>(data);}


} // namespace
