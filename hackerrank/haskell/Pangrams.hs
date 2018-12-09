import Data.Char

alphabet :: String
alphabet = "abcdefghijklmnopqrstuvwxyz"

isPangram :: String -> Bool
isPangram s = all (`elem` lower) alphabet
  where
    lower = map toLower s

solve :: String -> String
solve s =
  if isPangram s
    then "pangram"
    else "not pangram"

main :: IO ()
main = interact solve
