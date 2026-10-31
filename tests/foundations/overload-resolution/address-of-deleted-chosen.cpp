// tidal-rules: DEL-1 OVL-37 OVL-12
// tidal-expect: reject
// tidal-error-clang: overload resolution selected deleted operator .&.
// tidal-error-gcc: use of deleted function
// tidal-error-edg: function .*operator&.* cannot be referenced -- it is a deleted function
// tidal-note: The deleted operator& is viable and wins, so the built-in meaning does not apply.
struct Item { void operator&() const = delete; };
Item v;
auto p = &v;
