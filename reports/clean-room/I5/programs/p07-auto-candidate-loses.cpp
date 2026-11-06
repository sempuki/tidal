// notes 3: a candidate whose result type comes from its body loses, so the body is not instantiated.
template <class T> auto f(T t) { return t.missing; }
int f(int) { return 0; }
int main() { return f(1); }
