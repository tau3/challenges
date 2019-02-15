import Control.Monad

solve :: [Int] -> Int
solve [from, to] = to' - from' + 1
  where
    from' = ceiling $ sqrt (fromIntegral from :: Double)
    to' = floor $ sqrt (fromIntegral to :: Double)
solve _ = error "incorrect input"

main :: IO ()
main = do
  q <- read <$> getLine
  bounds <- replicateM q (map read . words <$> getLine)
  mapM_ (print . solve) bounds
