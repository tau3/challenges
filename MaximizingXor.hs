import Control.Monad.ST
import Data.Bits
import Data.Foldable
import Data.STRef

solve :: Int -> Int -> Int
solve l r =
  runST $ do
    res <- newSTRef 0
    for_ [l .. r] $ \i ->
      for_ [l .. r] $ \j -> do
        let current = i `xor` j
        modifySTRef'
          res
          (\m ->
             if m < current
               then current
               else m)
    readSTRef res

main :: IO ()
main = interact $ show . (\(l:r:_) -> solve l r) . map read . lines
