// tidal-rules: TARG-17 TARG-10 DEP-25
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction member_template_of_specialization_as_template_argument.cpp and probes member_template_of_plain_class_as_template_argument.cpp and member_template_of_specialization_named.cpp (from strong_second_try.cpp). Policy<char>::holder names a member class template of a class template specialization with no argument list. The parameter H of Store is a template template parameter, so the argument is a template (TARG-17). TARG-10 applies where the program needs a type or a value, and a template template argument is neither. In the alias template, P::template holder is the same argument in a dependent form (DEP-25).
template <class Unit> struct Policy {
  template <class V> struct holder { V value = 2; };
};
template <template <class> class H> struct Store { H<int> slot; };
static_assert(sizeof(Store<Policy<char>::holder>) == sizeof(int));
template <class P> using held = Store<P::template holder>;
static_assert(sizeof(held<Policy<long>>) == sizeof(int));
int main() {
  Store<Policy<char>::holder> s;
  held<Policy<short>> t;
  return s.slot.value + t.slot.value == 4 ? 0 : 1;
}
