#ifndef TIDAL_ODR_LIMIT_TEMPLATE_H
#define TIDAL_ODR_LIMIT_TEMPLATE_H
template <class T> const T lower = T(5);
template <class T> constexpr T upper = T(6);
const int* lower_elsewhere();
const int* upper_elsewhere();
#endif
