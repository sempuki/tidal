// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-error-clang: use .template. keyword to treat .from_base. as a dependent template name
// tidal-error-gcc: expected primary-expression before
// tidal-error-edg: type name is not allowed
// tidal-note: Q-0427, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-dep-24-object-forms.json. The object w has the type of the current instantiation, but the member template from_base is declared only in the dependent base Base<T>. Lookup at the definition does not find it in Wrap, so DEP-24 asks for template, and without it the < is the less-than operator. Every compiler rejects.
template <class T> struct Base {
  template <class U> U from_base() const { return U(2); }
};
template <class T> struct Wrap : Base<T> {
  static int read(const Wrap& w) { return w.from_base<int>(); }
};
int main() {
  Wrap<double> w;
  return Wrap<double>::read(w) == 2 ? 0 : 1;
}
