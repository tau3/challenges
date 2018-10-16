cycles :: String -> [Int]
cycles = map read . tail . words

utopianTree :: Int -> Int
utopianTree x
  | x == 0 = 1
  | odd x = 2 * utopianTree (x - 1)
  | otherwise = 1 + utopianTree (x - 1)

main :: IO ()
main = interact $ unwords . map (show . utopianTree) . cycles
