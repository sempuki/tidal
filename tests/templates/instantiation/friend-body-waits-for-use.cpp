// tidal-rules: INST-5
// tidal-expect: accept
// tidal-note: I6-16. The instantiation of Wrap<int> declares the friend function touch, and its body waits for a use. No call uses touch, so the body, which is not valid for int, is not instantiated.
template <class T> struct Wrap {
  friend void touch(Wrap) { T::missing(); }
};
Wrap<int> held;
