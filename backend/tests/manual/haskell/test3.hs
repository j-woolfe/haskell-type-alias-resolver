type TestAlias = Int
type TestAliasDupe = Int
type TestAliasString = String
type TestVariable String = Maybe String
type TestList = [Char]
type TestNestedList = [[Char]]
type TestTuple = (String, Int)
type TestTupleNested = (String, (Int, Int))
type TestUnit = ()

type TestFunc = Int -> Int
type TestFuncDupe = Int -> Int
type TestFuncString = String -> String
type TestFuncTrip = Int -> String -> Int
type TestFunTuple = (Int, Int) -> String -> (String, (Int, Int))

type TestVar a = a -> a
type TestVarMixed a = a -> Int
type TestVarDiff a b = a -> b
type TestVarTuple a b = (a, a) -> b -> (b, (a, a))

