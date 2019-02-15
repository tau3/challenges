import Data.Maybe (fromJust, isJust)
import qualified Data.Set as S (Set, fromList, lookupGE, lookupLE)

distance :: S.Set Int -> Int -> Int
distance cs i =
  minimum $
  map (abs . (+ (-i)) . fromJust) $
  filter isJust [S.lookupLE i cs, S.lookupGE i cs]

solve :: Int -> [Int] -> Int
solve n c = maximum $ map (distance cs) [0 .. n - 1]
  where
    cs = S.fromList c

main :: IO ()
main = do
  [n, _] <- map read . words <$> getLine
  c <- map read . words <$> getLine
  print $ solve n c
