type TestAlias = Int
type TestGhost a = String
type TestVariable a = Maybe a
type TestList = [Char]
type TestListVar a = [a]
type TestNestedList = [[Char]]
type TestTuple = (String, Int)
type TestTupleMixed a = (String, a)
type TestTupleVars a b = (a, b)
type TestTupleVars a = (a, a)
type TestUnit = ()

type TestFunc = Int -> Int

