template <class T> struct Box {
  template <class K> void by(K T::* key);
};
Box<int> b;
int main() { return 0; }
