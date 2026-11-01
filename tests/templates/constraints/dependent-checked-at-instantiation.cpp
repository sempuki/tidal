// tidal-rules: CNS-13 CNS-14
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for
// tidal-error-gcc: template constraint failure|invalid variable template|placeholder constraints
// tidal-error-edg: template constraint not satisfied
// tidal-note: User<char> is instantiated, so Box<char> is named with arguments that fail.
template <class T> requires (sizeof(T) > 1) struct Box {};
template <class T> struct User { Box<T>* p; };
User<char> user;
