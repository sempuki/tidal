// tidal-rules: ACC-20
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: inaccessible
// tidal-note: Q-0391. The friend declaration names peek in every specialization of Box, explicit specializations included, so Box<char>::peek reads the private member. Clang and GCC accept. EDG rejects the access from the explicit specialization.
template <class T> struct Box { int peek(); };
class Safe {
  int secret = 7;
  template <class T> friend int Box<T>::peek();
};
Safe s;
template <> struct Box<char> { int peek() { return s.secret; } };
int main() { return Box<char>().peek() == 7 ? 0 : 1; }
