#ifndef TIDAL_ODR_GUARD_NESTED_H
#define TIDAL_ODR_GUARD_NESTED_H
#ifndef TIDAL_ODR_NESTED_LIMIT
#define TIDAL_ODR_NESTED_LIMIT 4
#endif
#if TIDAL_ODR_NESTED_LIMIT > 2
struct Nested { int v = TIDAL_ODR_NESTED_LIMIT; };
#else
struct Nested { int v = 0; };
#endif
#endif
