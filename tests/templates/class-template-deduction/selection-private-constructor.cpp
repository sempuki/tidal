// tidal-rules: CTAD-26 CTAD-17 CTAD-25
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: The guide from the private constructor is chosen, because guides ignore access. Deduction gives Secret<int>, and the initialization then cannot use the private constructor.
template <class T> class Secret { Secret(T*) {} public: template <class U> Secret(U) {} };
int n = 0;
Secret s(&n);
