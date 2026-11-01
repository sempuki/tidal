// tidal-rules: INST-8
// tidal-expect: run
int calls = 0;
int bump() { return ++calls; }
template <class T> struct Box { static int count; };
template <class T> int Box<T>::count = bump();
Box<int> b;
int main() { return calls == 0 ? 0 : 1; }
