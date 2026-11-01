// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-error-clang: expected expression|use 'template' keyword
// tidal-error-gcc: expected primary-expression
// tidal-error-edg: type name is not allowed|expected an expression
struct Has { template <class U> int get() { return 1; } };
template <class T> int run(T* object) { return object->get<int>(); }
Has h;
int x = run(&h);
