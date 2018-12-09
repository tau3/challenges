import Data.List

median :: [Int] -> Float
median [] = error "empty list"
median [x] = fromIntegral x
median xs =
  let xs' = sort xs
      l = length xs'
      half = l `div` 2
   in if odd l
        then fromIntegral $ xs' !! half
        else fromIntegral (xs !! half + xs !! (half - 1)) / 2

solve :: [Int] -> Int -> Int
solve ex d = solve' d 0
  where
    solve' i res
      | i == length ex = res
      | otherwise =
        if fromIntegral (ex !! i) >= (m * 2)
          then solve' (i + 1) (res + 1)
          else solve' (i + 1) res
      where
        look = take d $ drop (i - d) ex
        m = median look

main :: IO ()
main = do
  [_, d] <- map read . words <$> getLine :: IO [Int]
  ex <- map read . words <$> getLine :: IO [Int]
  print $ solve ex d
