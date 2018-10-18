{-# LANGUAGE ScopedTypeVariables #-}

import Control.Monad.ST (ST, runST)
import Data.Array (Array, (!))
import Data.Array.ST (STArray, freeze, newArray, readArray, writeArray)
import Data.Foldable (for_)

lcsBottomUp :: String -> String -> Int
lcsBottomUp xs ys =
  runST $ do
    let w = length xs
    let h = length ys
    arr <-
      newArray ((0, 0), (w + 1, h + 1)) 0 :: ST s (STArray s (Int, Int) Int)
    for_ [1 .. w] $ \i ->
      for_ [1 .. h] $ \j -> do
        val :: Int <-
          if (xs !! (i - 1)) == (ys !! (j - 1))
            then (+ 1) <$> readArray arr (i - 1, j - 1)
            else max <$> readArray arr (i - 1, j) <*> readArray arr (i, j - 1)
        writeArray arr (i, j) val
    res :: Array (Int, Int) Int <- freeze arr
    return $ res ! (w, h)

lcs _ [] = 0
lcs [] _ = 0
lcs l@(x:xs) r@(y:ys) =
  if x == y
    then 1 + lcs xs ys
    else max (lcs l ys) (lcs r xs)

main = return ()
