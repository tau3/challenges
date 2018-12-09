import qualified Data.Set as Set

minimumLoss :: [Int] -> Int
minimumLoss (y:ys) = go (Set.singleton y) ys maxBound
  where
    go _ [] res = res
    go prevs (r:rs) res = min res' (go (Set.insert r prevs) rs res')
      where
        maybePrice = Set.lookupGT r prevs
        res' = maybe res (\p -> min res (p - r)) maybePrice
minimumLoss _ = error "incorrect case"

main :: IO ()
main = (minimumLoss . map read . words <$> (getLine >> getLine)) >>= print
