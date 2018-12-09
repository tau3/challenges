jumps :: [Int] -> Int -> Int
jumps _ 0 = 0
jumps c n
  | n < 0 = 0
  | c !! n == 1 = length c
  | otherwise = 1 + min (jumps c (n - 1)) (jumps c (n - 2))

solve :: [Int] -> Int
solve xs = jumps xs (length xs - 1)

main :: IO ()
main = getLine >> solve . map read . words <$> getLine >>= print
