// A requires expression that names Box<T> gives false and no error.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> constexpr bool boxable = requires { typename Box<T>; };
static_assert(!boxable<char> && boxable<int>);
int main() {}
