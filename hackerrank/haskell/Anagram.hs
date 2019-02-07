import Control.Monad (replicateM_)
import Data.List (delete, splitAt)

splitInHalf :: [a] -> ([a], [a])
splitInHalf xs = splitAt ((length xs + 1) `div` 2) xs

countChanges :: Eq a => [a] -> [a] -> Int
countChanges xs (y:ys) = countChanges (delete y xs) ys
countChanges xs [] = length xs

solve :: Eq a => [a] -> Int
solve xs
  | even (length xs) = uncurry countChanges (splitInHalf xs)
  | otherwise = -1

main :: IO ()
main = do
  q <- read <$> getLine
  replicateM_ q (solve <$> getLine >>= print)
