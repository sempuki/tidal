// tidal-rules: ACC-20 ACC-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error: private|inaccessible
// tidal-note: Q-0391. Only peek of each specialization of Box is a friend, so Box<T>::other has no access. GCC and EDG reject. Clang warns that it does not support the friend form, turns off access checks for Safe, and accepts.
template <class T> struct Box { int peek(); int other(); };
class Safe {
  int secret = 7;
  template <class T> friend int Box<T>::peek();
};
Safe s;
template <class T> int Box<T>::peek() { return s.secret; }
template <class T> int Box<T>::other() { return s.secret; }
int main() { return Box<int>().other(); }
