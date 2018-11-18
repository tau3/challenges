solve :: String -> Int
solve s =
  if isPalindrome s
    then -1
    else firstMatch' s 0
  where
    firstMatch' xs i
      | i == length xs = -1
      | otherwise =
        if isPalindrome (xs `chop` i)
          then i
          else firstMatch' xs (i + 1)

isPalindrome :: String -> Bool
isPalindrome s = s == reverse s

chop :: String -> Int -> String
chop xs i = take i xs ++ drop (i + 1) xs

main :: IO ()
main = interact $ unlines . map (show . solve) . tail . lines
