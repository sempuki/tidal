template <class T> int size(T value) { return sizeof(value); }
template <class T> struct Box { int get(); static int count; };
template <class T> int Box<T>::get() { return 2; }
template <class T> int Box<T>::count = 3;
template <class T> int zero = 4;
template int size(int);
template struct Box<int>;
template int zero<int>;
