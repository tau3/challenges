import Data.Function
import Data.List

solve :: [Int] -> Int
solve = head . minimumBy (flip compare `on` length) . group . sort

main :: IO ()
main = getLine >> show . solve . map read . words <$> getLine >>= putStrLn
