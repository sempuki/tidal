// A partial specialization whose base is another specialization of the same
// template, named by its template-id in the mem-initializer. The front end
// reads the name as the class itself and rejects with CTOR-8.
template <class T> struct Box { int tag = 0; Box() = default; Box(int t) : tag(t) {} };
template <class T> struct Box<T*> : Box<int> {
  Box(int t) : Box<int>(t) {}
};
int main() {
  Box<char*> b(3);
  return b.tag == 3 ? 0 : 1;
}
