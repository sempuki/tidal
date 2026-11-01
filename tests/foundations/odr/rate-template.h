#ifndef TIDAL_ODR_RATE_TEMPLATE_H
#define TIDAL_ODR_RATE_TEMPLATE_H
template <class T> int rate = 1;
template <> int rate<char> = 2;
int rate_elsewhere();
#endif
