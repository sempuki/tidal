#ifndef TIDAL_ODR_COUNT_TEMPLATE_H
#define TIDAL_ODR_COUNT_TEMPLATE_H
template <class T> int digits(T) { return 0; }
template <> int digits(long);
int digits_elsewhere();
#endif
