// tidal-rules: AUTO-8 OVL-20
// tidal-expect: reject
// tidal-error-clang: incompatible initializer of type .<overloaded function type>
// tidal-error-gcc: unable to deduce .auto. from
// tidal-error-edg: cannot deduce .auto. type|overloaded|cannot
// tidal-note: A set of one function template, without template arguments that fix its parameter, leaves no function under OVL-20.
template <class T> void each(T);
auto p = each;
