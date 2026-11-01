// tidal-rules: CTAD-3
// tidal-expect: reject
// tidal-error-clang: deduced as 'Box<int>' in declaration of 'c' and deduced as 'Box<double>'
// tidal-error-gcc: inconsistent deduction for
// tidal-error-edg: type is "Box<double>" for this entity
template <class T> struct Box { T v; Box(T x) : v(x) {} };
Box a(1), b(2);
Box c(1), d(2.0);
