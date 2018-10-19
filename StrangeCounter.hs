import Control.Monad (join)

xs :: [Int]
xs = join $ map (takeWhile (> 0) . iterate (\y -> y - 1)) $ iterate (* 2) 3

main :: IO ()
main = interact $ show . (\pos -> xs !! (pos - 1)) . read
