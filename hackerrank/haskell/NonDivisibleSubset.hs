import Control.Monad (when)
import qualified Control.Monad.State as S (modify, runStateT)
import Data.List (foldl')
import qualified Data.Vector as V (Vector, (!), modify, replicate)
import qualified Data.Vector.Mutable as MV (modify)

makeRems :: [Int] -> Int -> V.Vector Int
makeRems s k =
  foldl' (\acc i -> increaseByOne acc (i `mod` k)) (V.replicate k 0) s
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
  [_, k] <- map read . words <$> getLine
  s <- map read . words <$> getLine
  let rems = makeRems s k
  ans <-
    snd <$>
    S.runStateT
      (do when (rems V.! 0 > 0) (S.modify (+ 1))
          when (even k && (rems V.! (k `div` 2) > 0)) (S.modify (+ 1)))
      (findAnswer rems k)
  print ans
