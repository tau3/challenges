import Control.Monad (replicateM_)
import Data.Foldable (foldl')

lowest :: Int
lowest = -(10 ^ 4) - 1

subArray :: [Int] -> Int
subArray xs = fst $ foldl' f (lowest, 0) xs
  where
    f (res, current) x = (res', current'')
      where
        current' = current + x
        res' = max res current'
        current'' = max 0 current'

subSequence :: [Int] -> Int
subSequence xs
  | null xs' = maximum xs
  | otherwise = sum xs'
  where
    xs' = filter (> 0) xs

main :: IO ()
main = do
  t <- read <$> getLine
  replicateM_ t $ do
    _ <- getLine
    arr <- map read . words <$> getLine
    putStrLn $ show (subArray arr) ++ " " ++ show (subSequence arr)
