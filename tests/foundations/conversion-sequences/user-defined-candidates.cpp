// tidal-rules: ICS-4
// tidal-expect: accept
// tidal-note: A conversion to a derived class serves a class target. Direct initialization adds explicit conversion functions to the class, and to a pointer that needs only a qualification adjustment.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Target {};
struct Derived : Target {};
struct Source { operator Derived() const; };
Target t = Source{};
struct Copyable { Copyable(); Copyable(const Copyable&); };
struct Maker { explicit operator Copyable() const; };
Copyable c(Maker{});
struct Address { explicit operator int*() const; };
const int* p(Address{});
Target shared;
struct Holder { operator Target&() const { return shared; } };
Target& r = Holder{};
