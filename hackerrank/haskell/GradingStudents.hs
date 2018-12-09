main :: IO ()
main = interact etl
  where
    etl = load . transform . extract
    extract = map read . tail . lines
    transform = map recalculate
    load = unlines . map show
    recalculate x
      | x < 38 = x
      | nextMultiple - x < 3 = nextMultiple
      | otherwise = x
      where
        nextMultiple = ((x `div` 5) + 1) * 5
