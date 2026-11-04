// tidal-rules: ICS-41 ICS-10
// tidal-expect: accept
// tidal-note: The explicit object parameter of an assignment operator is an ordinary parameter, so a user-defined sequence from Target to Handle reaches it. ICS-10 limits only the built-in assignment.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Target;
struct Handle { Handle(Target&); };
struct Target {
  One operator=(this Handle, int);
  Two operator+=(this Handle, int);
};
Target t;
static_assert(same<decltype(t = 1), One>);
static_assert(same<decltype(t += 1), Two>);
