import Data.Foldable

import Control.Monad
import Data.IORef
import Data.List

whenM :: Monad m => m Bool -> m () -> m ()
whenM b t = do
  b' <- b
  when b' t

elemIO :: Eq a => a -> IORef [a] -> IO Bool
elemIO c str = do
  r <- readIORef str
  return $ c `elem` r

solve :: String -> IO Int
solve str = do
  let l = length str
  if even l
    then (do let half = (1 + l) `div` 2
             left <- newIORef (take half str)
             let right = drop half str
             for_ [0 .. length right - 1] $ \i -> do
               let c = right !! i
               whenM (c `elemIO` left) $ modifyIORef' left (delete c)
             length <$> readIORef left)
    else return (-1)

main :: IO ()
main = do
  q <- read <$> getLine :: IO Int
  for_ [1 .. q] $ \_ -> do
    s <- getLine
    res <- solve s
    print res
