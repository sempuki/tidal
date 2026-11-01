// tidal-rules: CNS-7
// tidal-expect: accept
template <class T> constexpr bool wide = sizeof(T) > 1;
template <class T> struct Traits {
  static constexpr bool yes = true;
  static const bool no = false;
};
template <class T> concept Wide = wide<T>;
template <class T> concept Checked = Traits<T>::yes && !Traits<T>::no;
template <class T> concept Converted = static_cast<const bool>(sizeof(T) > 0);
static_assert(Wide<int> && !Wide<char> && Checked<int> && Converted<int>);
