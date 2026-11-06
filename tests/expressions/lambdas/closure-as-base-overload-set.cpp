// tidal-rules: LAM-29 PACK-21 CTAD-27 OVL-27
// tidal-expect: run
// tidal-note: Write suite W18, finding W18-9 (reports/clean-room/W18/report.md). A class template derives from a pack of closure types and brings in each call operator. A plain lambda for int wins over a generic one (OVL-27), and aggregate deduction gives the bases.
template <class... Fs> struct Overload : Fs... { using Fs::operator()...; };
int main() {
  Overload o{[](int) { return 1; }, [](auto) { return 2; }};
  return o(0) == 1 && o(0.5) == 2 ? 0 : 1;
}
