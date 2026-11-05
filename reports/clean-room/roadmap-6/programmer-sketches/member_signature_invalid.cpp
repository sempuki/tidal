template <class T> struct Box {
  void by(int T::* key);
};
Box<int> b;
int main() { return 0; }
