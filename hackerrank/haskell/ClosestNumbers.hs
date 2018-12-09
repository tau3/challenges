import Data.Function
import Data.List

tuplesToList :: [(a, a)] -> [a]
tuplesToList ((a, b):xs) = a : b : tuplesToList xs
tuplesToList _ = []

solve :: [Int] -> [Int]
solve =
  tuplesToList .
  head .
  groupBy ((==) `on` diff) . sortBy (compare `on` diff) . (zip <*> tail) . sort
  where
    diff t = abs $ uncurry (-) t

main :: IO ()
main = interact $ unwords . map show . solve . map read . words . last . lines
