solve :: String -> String
solve xs
  | null remains = "YES"
  | otherwise = "NO"
  where
    remains = solve' xs "hackerrank"

solve' :: String -> String -> String
solve' [] ys = ys
solve' _ [] = []
solve' (x:xs) (y:ys)
  | x == y = solve' xs ys
  | otherwise = solve' xs (y : ys)

main :: IO ()
main = interact $ unlines . map solve . tail . lines
