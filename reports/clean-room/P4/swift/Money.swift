// A money type that counts cents, made from integer and decimal
// literals, with arithmetic and ordering. Exits 0 when every check
// holds.
import Foundation

struct Money: ExpressibleByIntegerLiteral, ExpressibleByFloatLiteral, Comparable, CustomStringConvertible {
    let cents: Int64

    init(cents: Int64) { self.cents = cents }
    init(integerLiteral dollars: Int64) { cents = dollars * 100 }
    init(floatLiteral dollars: Double) { cents = Int64((dollars * 100).rounded()) }

    static func + (a: Money, b: Money) -> Money { Money(cents: a.cents + b.cents) }
    static func - (a: Money, b: Money) -> Money { Money(cents: a.cents - b.cents) }
    static func * (a: Money, n: Int64) -> Money { Money(cents: a.cents * n) }
    static func < (a: Money, b: Money) -> Bool { a.cents < b.cents }

    func split(_ parts: Int64) -> [Money] {
        let share = cents / parts
        let extra = cents % parts
        return (0..<parts).map { Money(cents: share + ($0 < extra ? 1 : 0)) }
    }

    var description: String {
        let sign = cents < 0 ? "-" : ""
        let magnitude = abs(cents)
        let rest = magnitude % 100
        return "\(sign)$\(magnitude / 100).\(rest < 10 ? "0" : "")\(rest)"
    }
}

var failures = 0
func check(_ ok: Bool) { if !ok { failures += 1 } }

let price: Money = 19.99
let fee: Money = 2
check(price.cents == 1999)
check(fee.cents == 200)

let total = price * 3 + fee
check(total.cents == 6197)
check(total.description == "$61.97")
check((fee - price).description == "-$17.99")

let shares = Money(cents: 1000).split(3)
check(shares.map { $0.cents } == [334, 333, 333])
check(shares.reduce(0, +) == 10)

let sorted = [Money(cents: 500), 1.5, 3].sorted()
check(sorted.map { $0.cents } == [150, 300, 500])
check(max(price, 20) == 20)

exit(failures == 0 ? 0 : 1)
