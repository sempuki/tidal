// tidal-rules: DED-14 DED-35
// tidal-expect: reject
// tidal-error-clang: does not match adjusted type
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: I5-8. T = int gives const int**, which int** does not reach by a qualification adjustment. The pair fails, so the call has no candidate. Clang's note names the deduced type and the argument type, and GCC's note names the cv-qualifiers.
template <class T> void loose(const T**);
int** pp = nullptr;
void use() { loose(pp); }
