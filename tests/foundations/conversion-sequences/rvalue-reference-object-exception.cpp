// tidal-rules: ICS-28 ICS-40
// tidal-expect: accept
// tidal-note: Item's operator has no ref-qualifier, so ICS-28 does not apply to its object parameter, the first operands tie, and the second decides. Fixed's operator has a ref-qualifier, so ICS-28 prefers the rvalue reference.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item { One operator+(int) const; };
Two operator+(const Item&&, long);
static_assert(same<decltype(Item{} + 0), One>);
struct Fixed { One operator+(int) const &; };
Two operator+(const Fixed&&, int);
static_assert(same<decltype(Fixed{} + 0), Two>);
