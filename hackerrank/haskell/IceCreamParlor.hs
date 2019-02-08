import Control.Monad (replicateM_)
import qualified Data.HashMap.Strict as Map
import Data.List (foldl')

solve :: [Int] -> Int -> [Int]
solve cs m = solve' enumerated
  where
    enumerated = zip [1 ..] cs
    cache = foldl' (\acc (i, c) -> Map.insert c i acc) Map.empty enumerated
    solve' [] = error "no solution"
    solve' ((i, x):xs) =
      case Map.lookup (m - x) cache of
        Just i' ->
          if (i /= i')
            then [i, i']
            else solve' xs
        Nothing -> solve' xs

main :: IO ()
main = do
  t <- read <$> getLine
  replicateM_ t $ do
    m <- read <$> getLine
    _ <- getLine
    cs <- map read . words <$> getLine
    putStrLn $ unwords $ map show $ solve cs m
