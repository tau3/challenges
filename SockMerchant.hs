import Data.List (group, sort)

solve :: [Int] -> Int
solve = sum . map (\l -> length l `div` 2) . group . sort

etl :: String -> String
etl = show . solve . map read . words . last . lines

main :: IO ()
main = interact etl
