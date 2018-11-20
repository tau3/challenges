import Data.Foldable

readIntList :: IO [Int]
readIntList = map read . words <$> getLine

main :: IO ()
main = do
  [_, t] <- readIntList
  width <- readIntList
  for_ [1 .. t] $ \_ -> do
    [i, j] <- readIntList
    let subList = take (j - i + 1) $ drop i width
    print $ minimum subList
