// tidal-rules: CTAD-3
// tidal-expect: reject
// tidal-error-clang: deduced as 'Box<int>' in declaration of 'e' and deduced as 'Box<char>'
// tidal-error-gcc: inconsistent deduction for
// tidal-error-edg: type is "Box<char>" for this entity
// tidal-note: The first declarator deduces Box<int> and the second Box<char>.
template <class T> struct Box { T v; Box(T x) : v(x) {} };
Box e = 1, f = 'c';
