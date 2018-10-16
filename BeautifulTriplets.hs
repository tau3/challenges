isStartOfBeautifulTriplet :: Int -> Int -> [Int] -> Bool
isStartOfBeautifulTriplet i d arr = (i + d) `elem` arr && (i + 2 * d) `elem` arr

countBeautifulTriplets d arr@(x:xs)
  | length xs < 3 = 0
  | otherwise =
    if isStartOfBeautifulTriplet x d arr
      then 1 + rest
      else rest
  where
    rest = countBeautifulTriplets d xs

main :: IO ()
main = do
  d <- (read . last . words <$> getLine)
  arr <- (map read . words <$> getLine)
  print $ countBeautifulTriplets d arr
