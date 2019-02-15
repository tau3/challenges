import Control.Monad

solve :: [Int] -> Bool
solve [] = False
solve [_] = True
solve (y:ys) = go 0 (y : ys) (sum ys)
  where
    go l (x1:x2:xs) r
      | l == r = True
      | otherwise = go (l + x1) (x2 : xs) (r - x2)
    go _ [] _ = False
    go l [_] _ = l == 0

main :: IO ()
main = do
  t <- read <$> getLine
  replicateM_ t $ do
    _ <- getLine
    arr <- map read . words <$> getLine
    putStrLn $
      if solve arr
        then "YES"
        else "NO"
