import Control.Monad
import Control.Monad.State
import Data.List (foldl')
import qualified Data.Vector as V
import qualified Data.Vector.Mutable as MV

makeRems :: [Int] -> Int -> V.Vector Int
makeRems s k =
  foldl'
    (\acc i ->
       let pos = (s !! i) `mod` k
        in increaseByOne acc pos)
    (V.replicate k 0)
    [0 .. length s - 1]
  where
    increaseByOne v pos = V.modify (\mw -> MV.modify mw (+ 1) pos) v

findAnswer :: V.Vector Int -> Int -> Int
findAnswer rems k = go 0 0
  where
    lim = ((k - 1) `div` 2) - 1
    go ans i
      | i > lim = ans
      | otherwise = go (ans + reminder) (i + 1)
      where
        reminder = max (rems V.! (i + 1)) (rems V.! (k - 1 - i))

main :: IO ()
main = do
  [_, k] <- map read . words <$> getLine :: IO [Int]
  s <- map read . words <$> getLine :: IO [Int]
  let rems = makeRems s k
  ans <-
    snd <$>
    runStateT
      (do when (rems V.! 0 > 0) (modify (+ 1))
          when (even k && (rems V.! (k `div` 2) > 0)) (modify (+ 1)))
      (findAnswer rems k)
  print ans
