import Control.Monad (forM_)

solve :: [Int] -> String
solve arr =
  if odd $ countTurns arr
    then "BOB"
    else "ANDY"

countTurns :: [Int] -> Int
countTurns arr = go arr 0 0
  where
    go (x:xs) localMax res =
      if x > localMax
        then go xs x (res + 1)
        else go xs localMax res
    go [] _ res = res

main :: IO ()
main = do
  g <- read <$> getLine :: IO Int
  forM_ [1 .. g] $ \_ -> do
    _ <- getLine
    arr <- map read . words <$> getLine :: IO [Int]
    putStrLn $ solve arr
