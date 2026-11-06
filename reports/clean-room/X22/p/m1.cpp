// The class User<char> is instantiated. Box<T> in the body of a member
// that is never called is not checked.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> struct User {
  int size() { return sizeof(Box<T>*); }
  int n = 0;
};
User<char> u;
int main() { return u.n; }
