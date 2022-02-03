module Task where

type JPair = (String, JValue)

data JValue = JString String
            | JNumber Double
            | JBool Bool
            | JNull
            | JObject [(String, JValue)]
            | JArray [JValue]

renderJValue :: JValue -> String
renderJValue (JString s) = show s
renderJValue (JNumber n) = show n
renderJValue (JBool True) = "true"
renderJValue (JBool False) = "false"
renderJValue JNull = "null"

renderJValue (JObject o) = "{" ++ renderPairs o ++ "}"
renderJValue (JArray a) = "[" ++ renderPairs a ++ "]"

renderPair :: (String, JValue) -> String
renderPair (k, v) = show k ++ ": " ++ renderJValue v

renderPairs [] = ""
renderPairs [p] = renderPair p
renderPairs (p:ps) = renderPair p ++ ", " ++ renderPairs ps

renderArrayValues [] = ""
renderArrayValues [v] = renderJValue v
renderArrayValues (v:vs) = renderJValue v ++ ", " ++ renderArrayValues vs 

