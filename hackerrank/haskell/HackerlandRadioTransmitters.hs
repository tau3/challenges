{-# LANGUAGE FlexibleContexts #-}

import Control.Monad.State
import Data.List
import qualified Data.Vector as V

while :: Monad m => m Bool -> m () -> m ()
while p a = p >>= (\p' -> when p' $ a >> while p a)

solve :: [Int] -> Int -> Int -> Int
solve houses n k =
  snd $
  execState
    (while (getsJ (< n)) $ do
       setRes (+ 1)
       loc <- getsJ (\j -> xs V.! j + k)
       while (getsJ (\j -> (j < n) && (xs V.! j <= loc))) (setJ (+ 1))
       setJ (+ (-1))
       loc' <- getsJ (\j -> xs V.! j + k)
       while (getsJ (\j -> (j < n) && (xs V.! j <= loc'))) (setJ (+ 1)))
    (0, 0)
  where
    xs = V.fromList $ sort houses
    getsJ f = gets (\(j, _) -> f j)
    setRes f = modify (\(j, res) -> (j, f res))
    setJ f = modify (\(j, res) -> (f j, res))

main :: IO ()
main = do
  [n, k] <- map read . words <$> getLine
  x <- map read . words <$> getLine
  print $ solve x n k
