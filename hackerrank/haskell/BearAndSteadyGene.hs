import qualified Data.HashMap.Strict as M (HashMap, alter, empty, foldr, update)
import Data.Maybe (isNothing)
import qualified Data.Vector as V (Vector, (!), foldr, fromList)

countChars :: V.Vector Char -> M.HashMap Char Int
countChars = V.foldr (M.alter f) M.empty
  where
    f maybeVal
      | isNothing maybeVal = Just 1
      | otherwise = fmap (+ 1) maybeVal

isBalanced :: M.HashMap Char Int -> Int -> Bool
isBalanced m n = M.foldr (\v acc -> acc && (v <= n)) True m

solve :: String -> Int
solve gene = solve' (countChars s) 0 0 l
  where
    s = V.fromList gene
    l = length s
    n = l `div` 4
    solve' m i j res
      | (i >= l) || (j >= l) = res
      | not (isBalanced m n) =
        let m' = M.update (\v -> return (v - 1)) (s V.! j) m
            j' = j + 1
         in solve' m' i j' res
      | otherwise =
        let res' = min res (j - i)
            i' = i + 1
            m' = M.update (return . (+ 1)) (s V.! i) m
         in solve' m' i' j res'

main :: IO ()
main = (solve <$> (getLine >> getLine)) >>= print
