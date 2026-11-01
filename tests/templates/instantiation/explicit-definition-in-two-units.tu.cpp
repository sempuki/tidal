template <class T> int size(T value) { return sizeof(value); }
template int size(int);
int there() { return size(2); }
