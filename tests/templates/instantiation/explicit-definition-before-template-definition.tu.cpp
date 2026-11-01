template <class T> int size(T);
template int size(int);
template <class T> int size(T value) { return sizeof(value); }
