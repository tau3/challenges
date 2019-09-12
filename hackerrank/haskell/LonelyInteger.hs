import Data.Bits (xor)

solve :: [Int] -> Int
solve = foldr1 xor

main :: IO ()
main = do
  _ <- getLine
  a <- map read . words <$> getLine
  print $ solve a
