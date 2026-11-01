template <class T> struct Pair { T first; T second; };
int size_elsewhere() { return sizeof(Pair<int>); }
