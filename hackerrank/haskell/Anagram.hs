import Control.Monad (replicateM_)
import Data.List (delete, splitAt)

splitInHalf :: [a] -> ([a], [a])
splitInHalf xs = splitAt ((length xs + 1) `div` 2) xs

countChanges :: Eq a => [a] -> [a] -> Int
countChanges xs (y:ys) = solve (delete y xs) ys
countChanges xs [] = length xs

megaSolve :: Eq a => [a] -> Int
megaSolve xs =
  if even (length xs)
    then uncurry countChanges (splitInHalf xs)
    else -1

main :: IO ()
main = do
  q <- read <$> getLine
  replicateM_ q (megaSolve <$> getLine >>= print)
