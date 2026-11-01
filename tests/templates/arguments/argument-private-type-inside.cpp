// tidal-rules: TARG-26
// tidal-expect: accept
// tidal-note: Owner names its own private type as an argument.
template <class T> struct Box { T value; };
class Owner {
  struct Hidden { int x; };
  Box<Hidden> box;
public:
  static constexpr int size = sizeof(Box<Hidden>);
};
static_assert(Owner::size == sizeof(int));
