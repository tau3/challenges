-- import Control.Monad.Reader
solve :: [Int] -> Bool -> Int -> Int -> Int -> Int -> Int
solve _ False 0 _ e _ = e
solve c _ i k e n = solve c False i' k e' n
  where
    e' =
      if c !! i == 0
        then e
        else e - 2
    i' = (i + k) `mod` n

main :: IO ()
main = do
  [n, k] <- map read . words <$> getLine :: IO [Int]
  c <- map read . words <$> getLine :: IO [Int]
  print $ solve c True 0 k 100 n
