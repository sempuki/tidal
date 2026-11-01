// tidal-rules: TARG-12
// tidal-expect: accept
// tidal-note: Array-to-pointer, function-to-pointer, qualification, null pointer, and function pointer conversions of constant arguments.
template <const int* P> struct At {};
template <void (*F)()> struct Call {};
template <int* P> struct Raw {};
int values[3];
int value;
void plain();
void quiet() noexcept;
At<values> a;
At<&value> b;
Call<plain> c;
Call<&plain> d;
Call<quiet> e;
Raw<nullptr> f;
