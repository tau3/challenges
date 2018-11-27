import Data.Foldable

import Data.IORef
import Data.List

ifM :: Monad m => m Bool -> m a -> m a -> m a
ifM b t f = do
  b' <- b
  if b'
    then t
    else f

whenM :: Monad m => m Bool -> m () -> m ()
whenM b t = ifM b t (return ())

elemIO :: Char -> IORef String -> IO Bool
elemIO c str = do
  r <- readIORef str
  return $ c `elem` r

solve :: String -> IO Int
solve s = do
  let l = length s
  if even l
    then (do let half = (1 + l) `div` 2
             a <- newIORef (take half s)
             let b = drop half s
             for_ [0 .. length b - 1] $ \i -> do
               let c = b !! i
               whenM (c `elemIO` a) $ modifyIORef' a (delete c)
             length <$> readIORef a)
    else return (-1)

main :: IO ()
main = do
  q <- read <$> getLine :: IO Int
  for_ [1 .. q] $ \_ -> do
    s <- getLine
    res <- solve s
    print res
