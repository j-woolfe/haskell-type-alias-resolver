type State s a = s -> (s, a) 

getUserId :: UserData -> (UserData, Int) -- State UserData Int 
setUserID :: UserData -> (UserData, ()) -- State UserData () 

getItemId :: ItemData -> (ItemData, Int) -- State ItemData Int 
setItemId :: ItemData -> (ItemData, ()) -- State ItemData () 
