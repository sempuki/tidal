// Today the error in User<char> shows only at the instantiation, far from
// the template that named Box<T>.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> struct User { Box<T> b; };
template <class T> struct Outer { User<T> u; };
Outer<char> o;
