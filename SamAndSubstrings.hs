import Data.List (inits, tails)

substrings :: String -> [String]
substrings = filter (not . null) . concatMap inits . tails

eval :: String -> Int
eval = sum . map read . substrings

limit :: Int
limit = 10 ^ 9 + 7

etl :: String -> String
etl = show . flip mod limit . eval

main :: IO ()
main = interact etl
