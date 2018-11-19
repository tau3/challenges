import Control.Monad
import Data.Foldable (for_)
import Data.IORef
import qualified Data.Vector as V
import qualified Data.Vector.Mutable as MV
import Prelude hiding ((/))

maybeModifyIORef :: IORef Int -> Bool -> (Int -> Int) -> IO ()
maybeModifyIORef ref p f
  | p = void (modifyIORef ref f)
  | otherwise = return ()

(%) :: Int -> Int -> Int
(%) = mod

(/) :: Int -> Int -> Int
(/) = div

(+=) :: IORef Int -> Int -> IO ()
(+=) ref i = void (modifyIORef ref (+ i))

main :: IO ()
main = do
  [_, k] <- map read . words <$> getLine :: IO [Int]
  s <- map read . words <$> getLine :: IO [Int]
  remsIO <- MV.replicate k 0
  for_ [0 .. length s - 1] $ \i -> MV.unsafeModify remsIO (+ 1) ((s !! i) % k)
  rems <- V.unsafeFreeze remsIO
  ansIO <- newIORef (0 :: Int)
  for_ [0 .. ((k - 1) / 2) - 1] $ \i ->
    ansIO += max (rems V.! (i + 1)) (rems V.! (k - 1 - i))
  maybeModifyIORef ansIO (rems V.! 0 > 0) (+ 1)
  maybeModifyIORef ansIO (even k && (rems V.! (k / 2) > 0)) (+ 1)
  ans <- readIORef ansIO
  print ans
