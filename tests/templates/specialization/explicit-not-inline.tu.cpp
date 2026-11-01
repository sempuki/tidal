template <class T> inline int size(T) { return 0; }
template <> int size(char) { return 1; }
int other() { return size('b'); }
