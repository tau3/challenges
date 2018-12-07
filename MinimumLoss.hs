import qualified Data.Set as Set

minimumLoss :: [Int] -> Int
minimumLoss xs = go (Set.singleton (head xs)) 1 maxBound
  where
    l = length xs
    go prevs i res
      | i == l = res
      | otherwise = min res' (go (Set.insert current prevs) (i + 1) res')
      where
        current = xs !! i
        sell = Set.lookupGT current prevs
        res' =
          case sell of
            Nothing -> res
            Just p -> min res (p - current)

main :: IO ()
main = (minimumLoss . map read . words <$> (getLine >> getLine)) >>= print
