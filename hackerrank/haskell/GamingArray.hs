import Control.Monad (forM_)

solve :: [Int] -> String
solve arr =
  if odd $ countTurns arr
    then "BOB"
    else "ANDY"

countTurns arr = length $ takeWhile (not . null) $ iterate turn arr

turn :: [Int] -> [Int]
turn arr = takeWhile (/= m) arr
  where
    m = maximum arr

main :: IO ()
main = do
  g <- read <$> getLine :: IO Int
  forM_ [1 .. g] $ \_ -> do
    n <- read <$> getLine :: IO Int
    arr <- map read . words <$> getLine :: IO [Int]
    putStrLn $ solve arr
