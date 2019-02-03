import Control.Monad (forM_)
import Data.List (sort, sortOn)
import Data.Ord (Down(..))

solve :: [Int] -> [Int] -> Int -> String
solve a b k =
  if all (>= k) ps
    then "YES"
    else "NO"
  where
    a' = sort a
    b' = sortOn Down b
    ps = zipWith (+) a' b'

getIntList :: IO [Int]
getIntList = map read . words <$> getLine :: IO [Int]

main :: IO ()
main = do
  q <- read <$> getLine :: IO Int
  forM_ [1 .. q] $ \_ -> do
    [_, k] <- getIntList
    a <- getIntList
    b <- getIntList
    putStrLn $ solve a b k
