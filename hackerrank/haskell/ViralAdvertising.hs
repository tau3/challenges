liked :: Int -> Int
liked 1 = 2
liked x = div (liked (x - 1) * 3) 2

cumulative :: Int -> Int
cumulative 1 = 2
cumulative x = sum (map liked [1 .. x])

etl :: String -> String
etl = show . cumulative . read

main :: IO ()
main = interact etl
