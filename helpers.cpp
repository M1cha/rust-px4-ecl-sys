#include <helpers.h>
#include <new>

void px4_ecl_sys_helper_quatf_get(const matrix::Quatf& q, float (&arr)[4]) {
    arr[0] = q(0);
    arr[1] = q(1);
    arr[2] = q(2);
    arr[3] = q(3);
}

void px4_ecl_sys_helper_vector3f_new(matrix::Vector3f& v) {
    new(&v) matrix::Vector3f();
}

void px4_ecl_sys_helper_vector3f_del(matrix::Vector3f& v) {
    v.~Vector3f();
}

void px4_ecl_sys_helper_vector3f_get(const matrix::Vector3f& v, float (&arr)[3]) {
    arr[0] = v(0);
    arr[1] = v(1);
    arr[2] = v(2);
}

void px4_ecl_sys_helper_vector3f_set(matrix::Vector3f& v, const float *arr) {
    v(0) = arr[0];
    v(1) = arr[1];
    v(2) = arr[2];
}
