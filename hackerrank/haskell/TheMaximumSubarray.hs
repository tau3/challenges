import Control.Monad
import Control.Monad.ST
import Data.Array
import Data.Array.ST
import Data.Foldable

solve :: Array Int Int -> Int -> (Int, Int)
solve arr n =
  runST $ do
    dp <-
      newArray ((0, 0), (n - 1, n - 1)) (0, 0) :: ST s (STArray s (Int, Int) ( Int
                                                                             , Int))
    for_ [0 .. n - 1] $ \i ->
      for_ [i .. n - 1] $ \j -> do
        maxSubArray <-
          if i == j
            then return (arr ! j)
            else do
              maxPrevSub <- fst <$> readArray dp (i, j - 1)
              return $ (arr ! j) + maxPrevSub
        maxSubSeq <-
          if i == j
            then return (arr ! j)
            else do
              prevMaxSubSeq <- snd <$> readArray dp (i, j - 1)
              return $ max prevMaxSubSeq ((arr ! j) + prevMaxSubSeq)
        writeArray dp (i, j) (maxSubArray, maxSubSeq)
    maxSubArray <- maximum . map fst <$> getElems dp
    maxSubSeq <- snd <$> readArray dp (0, n - 1)
    return (maxSubArray, maxSubSeq)

main :: IO ()
main = do
  t <- read <$> getLine
  replicateM_ t $ do
    n <- read <$> getLine
    arr <- listArray (0, n - 1) . map read . words <$> getLine
    let res = solve arr n
    putStrLn $ show (fst res) ++ " " ++ show (snd res)
