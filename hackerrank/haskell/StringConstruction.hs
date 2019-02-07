import Control.Monad (replicateM_)
import Data.List (nub)

main :: IO ()
main = do
  n <- read <$> getLine :: IO Int
  replicateM_ n $ (length . nub <$> getLine) >>= print
