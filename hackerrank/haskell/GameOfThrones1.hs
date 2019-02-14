import Data.List

isAnagramOfPalindrome :: String -> Bool
isAnagramOfPalindrome = (< 2) . length . filter (odd . length) . group . sort

solve :: String -> String
solve xs
  | isAnagramOfPalindrome xs = "YES"
  | otherwise = "NO"

main :: IO ()
main = interact solve
