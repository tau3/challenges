import Control.Monad
import Data.Char

diffs :: String -> [Int]
diffs = map (abs . uncurry (-)) . (zip <*> tail) . map ord

isFunny :: String -> Bool
isFunny s = diffs s == diffs (reverse s)

main :: IO ()
main = do
  q <- read <$> getLine
  replicateM_
    q
    (do str <- getLine
        let msg =
              if isFunny str
                then "Funny"
                else "Not Funny"
        putStrLn msg)
