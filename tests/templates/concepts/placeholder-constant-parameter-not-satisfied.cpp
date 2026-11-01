// tidal-rules: CPT-22
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for class template 'Value'
// tidal-error-gcc: placeholder constraints not satisfied|template constraint failure
// tidal-error-edg: template constraint not satisfied
template <class T> concept Big = sizeof(T) > 1;
template <Big auto N> struct Value {};
Value<'a'> value;
