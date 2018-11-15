import Data.List

solve :: [Int] -> Int
solve = median . sort
  where
    median xs = xs !! (length xs `div` 2)

main :: IO ()
main = interact $ show . solve . map read . words . last . lines
