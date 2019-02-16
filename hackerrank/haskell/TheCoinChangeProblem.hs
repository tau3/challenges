import Control.Monad (forM_)
import Control.Monad.ST (ST, runST)
import qualified Data.Vector.Mutable as MV

solve :: [Int] -> Int -> Int
solve cs n =
  runST $ do
    dp <- MV.replicate (n + 1) 0 :: ST s (MV.STVector s Int)
    MV.write dp 0 1
    let m = length cs - 1
    forM_ [0 .. m] $ \i -> do
      let c = cs !! i
      forM_ [c .. n] $ \j -> do
        val <- MV.read dp (j - c)
        MV.modify dp (+ val) j
    MV.read dp n

main :: IO ()
main = do
  [n, _] <- map read . words <$> getLine
  cs <- map read . words <$> getLine
  print $ solve cs n
