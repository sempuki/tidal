template <class T> int size(T value) { return sizeof(value); }
template <> int size(int) { return 99; }
int there() { return size(2); }
