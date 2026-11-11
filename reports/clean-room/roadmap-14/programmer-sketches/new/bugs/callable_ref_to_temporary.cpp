// The FunctionRef bug that fnref.cpp nearly had: a reference to a member
// pointer temporary that ends with its full-expression.
struct Word { int len; int twice() const { return 2 * len; } };
struct Ref {
  const void* obj;
  int (*call)(const void*, const Word&);
  template <class M>
  Ref(const M& m) : obj(&m), call([](const void* o, const Word& w) { return (w.*(*static_cast<const M*>(o)))(); }) {}
  int operator()(const Word& w) const { return call(obj, w); }
};
int main() {
  Ref r = &Word::twice;  // the pointer to member is a temporary
  Word w{4};
  return r(w) == 8 ? 0 : 1;
}
