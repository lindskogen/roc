app "echo"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.7.0/bkGby8jb0tmZYsy2hg1E_B2QrCgcSTxdUlHtETwm5m4.tar.br" }
    imports [pf.Stdin, pf.Stdout, pf.Task.{ Task }]
    provides [main] to pf

main : Task {} I32
main =

    # v = Num.sqrt 5 # NumSqrtUnchecked
    v = Num.sqrtChecked 5
    # v = Num.pow 2 1.2 # NumPow

    Stdout.line "done: \(Inspect.toStr v)"
