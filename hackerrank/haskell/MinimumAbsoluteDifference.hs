import Data.List (sort)

solve :: (Ord a, Num a) => [a] -> a
solve = minimum . map absDiff . pairs . sort
  where
    pairs = zip <*> tail
    absDiff = abs . uncurry (-)

getInts :: IO [Int]
getInts = map read . words <$> getLine

main :: IO ()
main = (solve <$> (getLine >> getInts)) >>= print
