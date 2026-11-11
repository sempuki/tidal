// explicit(bool) on a converting constructor template.
template <class T> struct Box { T v; template <class U> explicit(sizeof(U) > sizeof(T)) Box(U u) : v(static_cast<T>(u)) {} };
int take(Box<long> b) { return static_cast<int>(b.v); }
int main() { Box<char> c(300L); return take(5) + (c.v == static_cast<char>(300L) ? 0 : 1) - 5; }
