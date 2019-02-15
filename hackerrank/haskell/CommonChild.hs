import Control.Monad (forM_)
import Control.Monad.ST (ST, runST)
import Data.Array (Array, (!), listArray)
import Data.Array.ST (STUArray, newArray, readArray, writeArray)

-- its VERY important for performance to use strict STUArray here (NOT STArray)
type Matrix s = STUArray s (Int, Int) Int

type CharArray = Array Int Char

stringToArray :: String -> CharArray
stringToArray xs = listArray (0, length xs - 1) xs

solve :: CharArray -> CharArray -> Int
solve xs ys =
  runST $ do
    let w = length xs
    let h = length ys
    dp <- newArray ((0, 0), (w + 1, h + 1)) 0 :: ST s (Matrix s)
    forM_ [1 .. w] $ \i ->
      forM_ [1 .. h] $ \j -> do
        val <-
          if xs ! (i - 1) == ys ! (j - 1)
            then (+ 1) <$> readArray dp (i - 1, j - 1)
            else max <$> readArray dp (i - 1, j) <*> readArray dp (i, j - 1)
        writeArray dp (i, j) val
    readArray dp (w, h)

main :: IO ()
main = do
  xs <- stringToArray <$> getLine
  ys <- stringToArray <$> getLine
  print $ solve xs ys
