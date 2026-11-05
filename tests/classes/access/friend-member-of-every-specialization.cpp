// tidal-rules: ACC-20
// tidal-expect: run
// tidal-note: The friend declarations with a template head name the member function peek and the member class Inner of every specialization of Box. Box<int>::peek and Box<long>::Inner::peek read the private member. Clang warns that it does not support the form and turns off access checks for Safe.
template <class T> struct Box { int peek(); struct Inner { int peek(); }; };
class Safe {
  int secret = 7;
  template <class T> friend int Box<T>::peek();
  template <class T> friend struct Box<T>::Inner;
};
Safe s;
template <class T> int Box<T>::peek() { return s.secret; }
template <class T> int Box<T>::Inner::peek() { return s.secret; }
int main() { return Box<int>().peek() + Box<long>::Inner().peek() == 14 ? 0 : 1; }
