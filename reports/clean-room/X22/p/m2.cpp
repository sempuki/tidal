// The same body, called: the check fails.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> struct User {
  int size() { return sizeof(Box<T>*); }
};
User<char> u;
int main() { return u.size(); }
