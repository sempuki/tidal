// tidal-rules: CTAD-29
// tidal-expect: accept
// tidal-note: Each guide from a constructor template keeps the constraint Wide<U>. For (1, 'a'), the guide from Box(U, T) has U = int and T = char, and its constraint holds. The guide from Box(T, U) has U = char, and its constraint fails, so the first guide gives Box<char>. A constrained class template keeps its constraint on its guides.
template <class T> concept Wide = sizeof(T) > 1;
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box {
  template <class U> requires Wide<U> Box(U, T) {}
  template <class U> requires Wide<U> Box(T, U) {}
};
Box box(1, 'a');
static_assert(same<decltype(box), Box<char>>);
template <class T> requires Wide<T> struct Pad { Pad(T) {} };
Pad pad(1);
static_assert(same<decltype(pad), Pad<int>>);
