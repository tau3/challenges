solve :: [Int] -> [Int] -> Int -> Int
solve ks ds b
  | null candidates = -1
  | otherwise = maximum candidates
  where
    pairs = [(k, d) | k <- ks, d <- ds]
    candidates = filter (<= b) $ map (uncurry (+)) pairs

main :: IO ()
main = do
  [b, _, _] <- readIntList
  keyboards <- readIntList
  drives <- readIntList
  let res = solve keyboards drives b
  print res
  where
    readIntList = map read . words <$> getLine :: IO [Int]
