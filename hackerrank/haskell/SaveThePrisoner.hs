import Data.Foldable

solve :: Int -> Int -> Int -> Int
-- solve n m s = drop (s - 1) (cycle [1 .. n]) !! (m - 1)
solve n m s = (s + m - 2) `mod` n + 1

main :: IO ()
main = do
  t <- read <$> getLine :: IO Int
  for_ [1 .. t] $ \_ -> do
    [n, m, s] <- map read . words <$> getLine
    print $ solve n m s
