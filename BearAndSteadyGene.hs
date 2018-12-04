import Data.List
import qualified Data.Map.Strict as Map
import Data.Maybe

countChars :: String -> Map.Map Char Int
countChars = foldl' (flip (Map.alter f)) Map.empty
  where
    f maybeVal
      | isNothing maybeVal = Just 0
      | otherwise = fmap (+ 1) maybeVal

isBalanced :: Map.Map Char Int -> Int -> Bool
isBalanced m n = Map.foldl (\acc v -> acc && (v <= n)) True m

solve :: String -> Int
solve s = solve' (countChars s) 0 0 l
  where
    l = length s
    n = l `div` 4
    solve' m i j res
      | (i >= l) || (j >= l) = res
      | not (isBalanced m n) =
        let m' = Map.update (\v -> return (v - 1)) (s !! j) m
            j' = j + 1
         in solve' m' i j' res
      | otherwise =
        let res' = min res (j - i)
            i' = i + 1
            m' = Map.update (return . (+ 1)) (s !! i) m
         in solve' m' i' j res'

main :: IO ()
main = (solve <$> getLine >> getLine) >>= print
