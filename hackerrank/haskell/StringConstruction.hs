import Control.Monad (mapM_, replicateM)
import Data.List (nub)

main :: IO ()
main =
  map (length . nub) <$> (read <$> getLine >>= (`replicateM` getLine)) >>=
  mapM_ print
