solve :: Int -> [Int] -> Int
solve k hs = max ((maximum hs) - k) 0

main :: IO ()
main = do
  let k = read . last . words <$> getLine
  solve <$> k <*> (map read . words <$> getLine) >>= print
