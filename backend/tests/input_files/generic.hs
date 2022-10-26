type GTag a = a
type GMaybe a = Maybe a
type GList a = [a]
type GVoid = ()

type G1Tuple a = (a)
type G2Tuple a b = (a, b)
type G2TupleMatching a = (a, a)
type G2TupleNested a b = (b, (a, a))
type GListTuples a b = [(a, b)]

type GFunctionBin a = a -> a
type GFunctionBinMixed a b = a -> b
type GFunctionMatching a b = a -> a -> b
type GFunctionNested a b = (a -> a) -> b
type GFunctionList a b = ([a] -> a) -> b
type GFunctionTuple a b c = (a, b) -> (a -> (b, c))

type GConcreteMixed a = a -> String -> a
type GConcreteMixed2 a b = a -> (b -> Bool) -> a

