// An unknown attribute on a function template and a class template.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <Wide T> [[checked]] int f(T) { Box<T> b{}; return sizeof b; }
template <Wide T> struct [[checked]] User { Box<T> b; };
int main() { User<int> u; return f(2) == sizeof(int) ? 0 : 1; }
