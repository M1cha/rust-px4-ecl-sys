#pragma once

#include <ekf.h>

void px4_ecl_sys_helper_quatf_get(const matrix::Quatf& q, float (&arr)[4]);

void px4_ecl_sys_helper_vector3f_new(matrix::Vector3f& v);
void px4_ecl_sys_helper_vector3f_del(matrix::Vector3f& v);
void px4_ecl_sys_helper_vector3f_get(const matrix::Vector3f& v, float (&arr)[3]);
void px4_ecl_sys_helper_vector3f_set(matrix::Vector3f& v, const float *arr);
