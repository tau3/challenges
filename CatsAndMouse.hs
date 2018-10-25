solve :: [Int] -> String
solve [x, y, z]
  | abs (x - z) < abs (y - z) = "Cat A"
  | abs (x - z) > abs (y - z) = "Cat B"
  | otherwise = "Mouse C"

main :: IO ()
main = do
  q <- read <$> getLine :: IO Int
  (map solve <$> (map (map read . words) <$> mapM (const getLine) [1 .. q])) >>=
    mapM_ putStrLn
